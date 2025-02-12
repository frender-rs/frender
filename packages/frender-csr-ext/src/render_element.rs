use std::{future::Future, pin::Pin, task::Poll};

use frender_html::{
    dom::{ui_handle::UiHandle as _, ProvideRenderContext},
    experimental::{PinnedRenderStateKind, PinnedRenderStateKindPollRender as _},
    CsrElement, RenderHtml, StateUnmount as _,
};

pin_project_lite::pin_project!(
    #[project = ElementOrStateProj]
    enum ElementOrState<E, S, UH> {
        Element {
            element: Option<E>,
        },
        State {
            #[pin]
            state: S,
            ui_handle: Option<UH>,
        },
    }
);

impl<E, S, UH> ElementOrState<E, S, UH> {
    fn as_pin_mut_state_or_insert<RI>(
        mut self: Pin<&mut Self>,
        render_init: impl FnOnce(E) -> (S, RI),
        init_pinned: impl FnOnce(RI, Pin<&mut S>) -> UH,
    ) -> (Pin<&mut S>, &mut UH) {
        match self.as_mut().project() {
            ElementOrStateProj::Element {
                element: element @ Some(_),
            } => {
                let element = element.take().unwrap();

                let (state, init) = render_init(element);
                self.set(Self::State {
                    state,
                    ui_handle: None,
                });

                let ElementOrStateProj::State {
                    mut state,
                    ui_handle,
                } = self.project()
                else {
                    unreachable!()
                };

                let ui_handle = ui_handle.insert(init_pinned(init, state.as_mut()));

                (state, ui_handle)
            }
            ElementOrStateProj::State {
                state: _,
                ui_handle: Some(_),
            } => {
                let ElementOrStateProj::State {
                    state,
                    ui_handle: Some(ui_handle),
                } = self.project()
                else {
                    unreachable!()
                };
                (state, ui_handle)
            }
            _ => unreachable!(),
        }
    }

    fn take_ui_handle_and_reset<Out>(
        mut self: Pin<&mut Self>,
        map_ui_handle: impl FnOnce(UH) -> Out,
    ) -> Out {
        let ElementOrStateProj::State {
            state: _,
            ui_handle,
        } = self.as_mut().project()
        else {
            unreachable!()
        };

        let out = map_ui_handle(ui_handle.take().unwrap());

        self.set(Self::Element { element: None });

        out
    }
}

pin_project_lite::pin_project!(
    pub struct RenderElement<
        P: ProvideRenderContext,
        E: CsrElement,
        Stop = std::future::Pending<()>,
    >
    where
        P::Renderer: RenderHtml,
    {
        p: P,
        #[pin]
        element_or_state: ElementOrState<
            E,
            <E::RenderStateKind as PinnedRenderStateKind>::PinnedState<P::Renderer>,
            <E::RenderStateKind as PinnedRenderStateKind>::PinnedUiHandle<P::Renderer>,
        >,
        #[pin]
        stop: Stop,
    }
);

impl<P: ProvideRenderContext, E: CsrElement> RenderElement<P, E>
where
    P::Renderer: RenderHtml,
{
    pub fn new(render_context: P, element: E) -> Self {
        Self::new_with_stop(render_context, element, std::future::pending())
    }
}

impl<P: ProvideRenderContext, E: CsrElement, Stop> RenderElement<P, E, Stop>
where
    P::Renderer: RenderHtml,
{
    pub fn new_with_stop(render_context: P, element: E, stop: Stop) -> Self {
        Self {
            p: render_context,
            element_or_state: ElementOrState::Element {
                element: Some(element),
            },
            stop,
        }
    }
}

impl<P: ProvideRenderContext, E: CsrElement, Stop: Future<Output = ()>> std::future::Future
    for RenderElement<P, E, Stop>
where
    P::Renderer: RenderHtml,
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        if let ElementOrState::Element {
            element: Option::None,
        } = &*this.element_or_state
        {
            return Poll::Ready(());
        }

        let (mut state, ui_handle) = this.element_or_state.as_mut().as_pin_mut_state_or_insert(
            |element| {
                let (state, init) = this.p.provide_render_context(|render_context| {
                    element.pinned_render_init(render_context)
                });
                (state, (init, &mut this.p))
            },
            |(init, this_p), state| {
                use frender_html::experimental::RenderInitPinned as _;
                this_p.provide_render_context(|render_context| {
                    init.render_init_pinned(render_context, state)
                })
            },
        );

        if let Poll::Pending = <E::RenderStateKind>::pinned_poll_render(
            this.p.renderer_mut(),
            state.as_mut(),
            ui_handle,
            cx,
        ) {
            return Poll::Pending;
        }

        if let Poll::Ready(()) = this.stop.poll(cx) {
            state.state_unmount();
            () = this
                .element_or_state
                .take_ui_handle_and_reset(|ui_handle| _ = ui_handle.unmount(this.p.renderer_mut()));
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
