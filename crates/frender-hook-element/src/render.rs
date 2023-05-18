use std::{marker::PhantomData, pin::Pin};

pub enum OptionRenderState<'a, S> {
    Uninitialized(Pin<&'a mut Option<S>>),
    Initialized {
        state: Pin<&'a mut S>,
        force_reposition: bool,
    },
}

pub struct RenderWith<
    F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
    S: frender_csr::RenderState,
> {
    f: F,
    _state: PhantomData<S>,
}

#[allow(non_snake_case)]
pub fn RenderWith<
    F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
    S: frender_csr::RenderState,
>(
    f: F,
) -> RenderWith<F, S> {
    RenderWith {
        f,
        _state: PhantomData,
    }
}

pub mod csr {
    use std::{marker::PhantomData, pin::Pin};

    use frender_csr::{CsrContext, Element, RenderState};

    use crate::OptionRenderState;

    pin_project_lite::pin_project!(
        pub struct State<S: RenderState> {
            #[pin]
            pub(super) inner: Option<S>,
        }
    );

    impl<S: RenderState> RenderState for State<S> {
        fn unmount(self: Pin<&mut Self>) {
            _ = self.project().inner.as_pin_mut().map(S::unmount)
        }

        fn state_unmount(self: Pin<&mut Self>) {
            _ = self.project().inner.as_pin_mut().map(S::state_unmount)
        }

        fn poll_csr(
            self: Pin<&mut Self>,
            ctx: &mut frender_csr::CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project()
                .inner
                .as_pin_mut()
                .map_or(std::task::Poll::Ready(()), |state| state.poll_csr(ctx, cx))
        }
    }

    pub struct CsrRenderContext<'a, 'ctx, State: RenderState> {
        context: &'a mut CsrContext<'ctx>,
        state: OptionRenderState<'a, State>,
    }

    impl<'a, 'ctx, State: RenderState> CsrRenderContext<'a, 'ctx, State> {
        pub(super) fn _new(
            context: &'a mut CsrContext<'ctx>,
            state: OptionRenderState<'a, State>,
        ) -> Self {
            Self { context, state }
        }

        pub fn render<E: Element<CsrState = State>>(
            self,
            element: E,
        ) -> crate::csr::Rendered<State> {
            match self.state {
                OptionRenderState::Uninitialized(mut state) => {
                    state.set(Some(element.into_csr_state(self.context)))
                }
                OptionRenderState::Initialized {
                    state,
                    force_reposition,
                } => {
                    #[cfg(debug_assertions)]
                    frender_csr::web_sys::console::log_2(
                        &"render_with::CsrRenderContext::render force_reposition".into(),
                        &force_reposition.into(),
                    );
                    element.update_csr_state_maybe_reposition(self.context, state, force_reposition)
                }
            }

            crate::csr::Rendered(PhantomData)
        }
    }
}

impl<
        F: FnOnce(csr::CsrRenderContext<'_, '_, S>) -> crate::csr::Rendered<S>,
        S: frender_csr::RenderState + Unpin,
    > frender_csr::Element for RenderWith<F, S>
{
    type CsrState = S;

    fn into_csr_state(self, ctx: &mut frender_csr::CsrContext) -> Self::CsrState {
        let mut state = None;
        let _: crate::csr::Rendered<S> = (self.f)(csr::CsrRenderContext::_new(
            ctx,
            OptionRenderState::Uninitialized(Pin::new(&mut state)),
        ));

        state.unwrap()
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut frender_csr::CsrContext,
        state: std::pin::Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        #[cfg(debug_assertions)]
        frender_csr::web_sys::console::log_2(
            &"RenderWith::update_csr_state_maybe_reposition".into(),
            &force_reposition.into(),
        );
        let _: crate::csr::Rendered<S> = (self.f)(csr::CsrRenderContext::_new(
            ctx,
            OptionRenderState::Initialized {
                state,
                force_reposition,
            },
        ));
    }
}
