use std::pin::Pin;

use crate::Element;

impl<R: Element> Element for Box<R> {
    type CsrState = R::CsrState;

    #[inline]
    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        R::into_csr_state(*self, ctx)
    }

    #[inline]
    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        R::update_csr_state(*self, ctx, state)
    }
}
