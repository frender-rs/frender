use std::pin::Pin;

use crate::RenderState;

pub trait Element {
    type CsrState: RenderState;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState;

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>);
}
