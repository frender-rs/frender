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

macro_rules! update_option {
    ($_self:ident . $method:ident ($ctx:ident, $state:ident $(, $arg:expr)? )) => {
        if let Some(this) = $_self {
            match $state.as_mut().as_pin_mut() {
                Some(state) => this.$method($ctx, state $(, $arg)?),
                None => $state.set(Some(this.into_csr_state($ctx))),
            };
        } else {
            <Option<E::CsrState> as RenderState>::unmount($state)
        }
    };
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
        update_option!(self.update_csr_state(ctx, state))
    }

    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        mut state: Pin<&mut Self::CsrState>,
    ) {
        update_option!(self.update_csr_state_force_reposition(ctx, state))
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        mut state: Pin<&mut Self::CsrState>,
        force_reposition: bool,
    ) {
        update_option!(self.update_csr_state_maybe_reposition(ctx, state, force_reposition))
    }
}
