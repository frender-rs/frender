pub struct BoxState<E>(pub E);

#[cfg(feature = "csr")]
mod csr {
    use std::pin::Pin;

    use frender_common::utils::pin_as_deref_mut;
    use frender_csr::Element;

    use super::BoxState;

    impl<E: Element> Element for BoxState<E> {
        type CsrState = Pin<Box<E::CsrState>>;

        #[inline]
        fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
            Box::pin(self.0.into_csr_state(ctx))
        }

        #[inline]
        fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
            E::update_csr_state(self.0, ctx, pin_as_deref_mut(state))
        }

        fn update_csr_state_force_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: Pin<&mut Self::CsrState>,
        ) {
            E::update_csr_state_force_reposition(self.0, ctx, pin_as_deref_mut(state))
        }

        fn update_csr_state_maybe_reposition(
            self,
            ctx: &mut frender_csr::CsrContext,
            state: Pin<&mut Self::CsrState>,
            force_reposition: bool,
        ) {
            E::update_csr_state_maybe_reposition(
                self.0,
                ctx,
                pin_as_deref_mut(state),
                force_reposition,
            )
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use std::pin::Pin;

    use frender_ssr::Element;

    use super::BoxState;

    impl<E: Element> Element for BoxState<E> {
        type SsrState = Pin<Box<E::SsrState>>;

        fn into_ssr_state(self) -> Self::SsrState {
            Box::pin(self.0.into_ssr_state())
        }
    }
}
