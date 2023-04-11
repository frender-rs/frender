use std::pin::Pin;

use crate::RenderState;

pub trait Element {
    type CsrState: RenderState;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState;

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>)
    where
        Self: Sized,
    {
        self.update_csr_state_maybe_reposition(ctx, state, false)
    }

    /// The element needs to be repositioned (re-add to the ctx)
    fn update_csr_state_force_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: Pin<&mut Self::CsrState>,
    ) where
        Self: Sized,
    {
        self.update_csr_state_maybe_reposition(ctx, state, true)
    }

    fn update_csr_state_maybe_reposition(
        self,
        ctx: &mut crate::CsrContext,
        state: Pin<&mut Self::CsrState>,
        force_reposition: bool,
    );
}
