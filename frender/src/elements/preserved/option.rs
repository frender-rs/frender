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

    pub fn update_or_into_state<E: frender_csr::Element>(
        element: E,
        mut state: Pin<&mut Option<E::CsrState>>,
        ctx: &mut frender_csr::CsrContext,
    ) {
        match state.as_mut().as_pin_mut() {
            Some(state) => element.update_csr_state(ctx, state),
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

        fn update_csr_state(
            self,
            ctx: &mut crate::CsrContext,
            mut state: Pin<&mut Self::CsrState>,
        ) {
            if let Some(element) = self.0 {
                update_or_into_state(element, state.as_mut().project().inner, ctx)
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
