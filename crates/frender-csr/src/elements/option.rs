use std::pin::Pin;

use crate::{Element, RenderState};

impl<S: RenderState> RenderState for Option<S> {
    fn unmount(mut self: Pin<&mut Self>) {
        let this = self.as_mut().as_pin_mut();
        match this {
            Some(state) => {
                S::unmount(state);
            }
            None => return,
        }
        self.set(None)
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Some(s) => S::poll_csr(s, ctx, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}

impl<E: Element> Element for Option<E> {
    type CsrState = Option<E::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        match self {
            Some(this) => Some(this.into_csr_state(ctx)),
            None => None,
        }
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, mut state: Pin<&mut Self::CsrState>) {
        if let Some(this) = self {
            match state.as_mut().as_pin_mut() {
                Some(state) => this.update_csr_state(ctx, state),
                None => state.set(Some(this.into_csr_state(ctx))),
            };
        } else {
            <Option<E::CsrState> as RenderState>::unmount(state)
        }
    }
}
