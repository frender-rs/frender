use std::pin::Pin;

use crate::{
    utils::{pin_as_deref_mut, pin_downcast_mut},
    Element,
};

pub struct BoxState<E, const DYN: bool = true>(pub E);

impl<E: Element> Element for BoxState<E, false> {
    type CsrState = Pin<Box<E::CsrState>>;

    #[inline]
    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        Box::pin(self.0.into_csr_state(ctx))
    }

    #[inline]
    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        E::update_csr_state(self.0, ctx, pin_as_deref_mut(state))
    }
}

impl<E: Element> Element for BoxState<E, true>
where
    E::CsrState: std::any::Any,
{
    type CsrState = Pin<Box<dyn crate::AnyRenderState<crate::CsrContext>>>;

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        let state = pin_downcast_mut(pin_as_deref_mut(state).pin_as_mut_any())
            .expect("state type should match");
        E::update_csr_state(self.0, ctx, state)
    }

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        Box::pin(E::into_csr_state(self.0, ctx))
    }
}
