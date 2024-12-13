use std::{future::Future, task::Poll};

use frender_html::{
    dom::{ui_handle::UiHandle as _, ProvideRenderContext},
    experimental::{
        PinMutRenderInitStates, PinnedRenderStateKind, PinnedRenderStateKindPollRender as _,
        RenderStates,
    },
    CsrElement, RenderHtml, StateUnmount as _,
};

enum ElementOrUiHandle<E, UH> {
    Taken,
    Element(E),
    UiHandle(UH),
}

impl<E, UH> ElementOrUiHandle<E, UH> {
    fn as_mut_ui_handle_or_insert(&mut self, f: impl FnOnce(E) -> UH) -> &mut UH {
        match self {
            ElementOrUiHandle::Taken => unreachable!(),
            ElementOrUiHandle::Element(_) => {
                let this = std::mem::replace(self, Self::Taken);
                let Self::Element(element) = this else {
                    unreachable!()
                };

                *self = Self::UiHandle(f(element));

                if let ElementOrUiHandle::UiHandle(this) = self {
                    this
                } else {
                    unreachable!()
                }
            }
            ElementOrUiHandle::UiHandle(this) => this,
        }
    }

    fn take_ui_handle(&mut self) -> UH {
        if let ElementOrUiHandle::UiHandle(this) = std::mem::replace(self, Self::Taken) {
            this
        } else {
            unreachable!()
        }
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
        element: ElementOrUiHandle<
            E,
            <E::RenderStateKind as PinnedRenderStateKind>::PinnedUiHandle<P::Renderer>,
        >,
        #[pin]
        non_reactive_state:
            <E::RenderStateKind as PinnedRenderStateKind>::PinnedNonReactiveState<P::Renderer>,
        #[pin]
        reactive_state: <E::RenderStateKind as PinnedRenderStateKind>::PinnedReactiveState,
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
            element: ElementOrUiHandle::Element(element),
            non_reactive_state: Default::default(),
            reactive_state: Default::default(),
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

        if let ElementOrUiHandle::Taken = this.element {
            return Poll::Ready(());
        }

        let ui_handle = this.element.as_mut_ui_handle_or_insert(|element| {
            this.p.provide_render_context(|render_context| {
                element.pinned_render_init(
                    render_context,
                    PinMutRenderInitStates {
                        non_reactive_state: this.non_reactive_state.as_mut(),
                        reactive_state: this.reactive_state.as_mut(),
                    },
                )
            })
        });

        if let Poll::Pending = <E::RenderStateKind>::pinned_poll_render(
            this.p.renderer_mut(),
            RenderStates {
                ui_handle,
                non_reactive_state: this.non_reactive_state.as_mut(),
                reactive_state: this.reactive_state.as_mut(),
            },
            cx,
        ) {
            return Poll::Pending;
        }

        if let Poll::Ready(()) = this.stop.poll(cx) {
            // The unmount order is the same as EitherElement accidentally
            // (This is not considered as a feature and might change in the future)
            this.non_reactive_state.set(Default::default());
            this.reactive_state.as_mut().state_unmount();
            this.reactive_state.set(Default::default());
            this.element.take_ui_handle().unmount(this.p.renderer_mut());
            return Poll::Ready(());
        }

        Poll::Pending
    }
}
