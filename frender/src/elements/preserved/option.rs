#[cfg(feature = "csr")]
pub mod csr {
    use std::pin::Pin;

    use frender_csr::{Element, RenderState};

    use super::super::Preserved;

    pin_project_lite::pin_project!(
        pub struct State<S> {
            #[pin]
            inner: Option<S>,
        }
    );

    pub(crate) fn update_with_or_into_state<E: frender_csr::Element>(
        element: E,
        mut state: Pin<&mut Option<E::CsrState>>,
        ctx: &mut frender_csr::CsrContext,
        update_with: impl FnOnce(E, &mut frender_csr::CsrContext, Pin<&mut E::CsrState>),
    ) {
        match state.as_mut().as_pin_mut() {
            Some(state) => update_with(element, ctx, state),
            None => state.set(Some(element.into_csr_state(ctx))),
        }
    }

    impl<S: RenderState> RenderState for State<S> {
        fn unmount(self: Pin<&mut Self>) {
            self.project().inner.as_pin_mut().map(S::unmount);
        }

        fn poll_csr(
            self: Pin<&mut Self>,
            ctx: &mut crate::CsrContext,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            self.project()
                .inner
                .as_pin_mut()
                .map_or(std::task::Poll::Ready(()), |s| S::poll_csr(s, ctx, cx))
        }
    }

    impl<E: Element> Element for Preserved<Option<E>> {
        type CsrState = State<E::CsrState>;

        fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
            State {
                inner: self.0.map(|this| E::into_csr_state(this, ctx)),
            }
        }

        fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
            self.update_with(ctx, state, E::update_csr_state)
        }

        fn update_csr_state_force_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: Pin<&mut Self::CsrState>,
        ) {
            self.update_with(ctx, state, E::update_csr_state_force_reposition)
        }

        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) {
            self.update_with(ctx, state, |element, ctx, state| {
                element.update_csr_state_maybe_reposition(ctx, state, force_reposition)
            })
        }
    }

    impl<E: Element> Preserved<Option<E>> {
        fn update_with(
            self,
            ctx: &mut frender_csr::CsrContext,
            mut state: Pin<&mut State<E::CsrState>>,
            with: impl FnOnce(E, &mut frender_csr::CsrContext, Pin<&mut E::CsrState>),
        ) {
            if let Some(element) = self.0 {
                update_with_or_into_state(element, state.as_mut().project().inner, ctx, with)
            } else {
                state.project().inner.as_pin_mut().map(E::CsrState::unmount);
            }
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_ssr::Element;

    impl<E: Element> Element for super::super::Preserved<Option<E>> {
        type SsrState = Option<E::SsrState>;

        fn into_ssr_state(self) -> Self::SsrState {
            self.0.into_ssr_state()
        }
    }
}
