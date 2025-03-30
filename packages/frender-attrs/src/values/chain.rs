use crate::Attributes;

pub struct ChainAttributes<A, B>(A, B);

impl<A: Attributes, B: Attributes> crate::sealed::Attributes for ChainAttributes<A, B> {}
impl<A: Attributes, B: Attributes> Attributes for ChainAttributes<A, B> {}

#[cfg(feature = "csr")]
mod csr {
    use crate::csr::{CsrAttributes, CsrAttributesStateUnmount};

    use super::ChainAttributes;

    pub struct ChainStates<A, B>(A, B);

    impl<A: CsrAttributesStateUnmount, B: CsrAttributesStateUnmount> CsrAttributesStateUnmount
        for ChainStates<A, B>
    {
        fn state_unmount(&mut self, renderer: &mut impl crate::csr::RenderAttributes) {
            self.0.state_unmount(renderer);
            self.1.state_unmount(renderer);
        }
    }

    impl<A: CsrAttributes, B: CsrAttributes> CsrAttributes for ChainAttributes<A, B> {
        type State = ChainStates<A::State, B::State>;

        fn render_init(self, renderer: &mut impl crate::csr::RenderAttributes) -> Self::State {
            ChainStates(self.0.render_init(renderer), self.1.render_init(renderer))
        }

        fn render_update(
            self,
            renderer: &mut impl crate::csr::RenderAttributes,
            ChainStates(state_a, state_b): &mut Self::State,
        ) {
            let Self(a, b) = self;
            a.render_update(renderer, state_a);
            b.render_update(renderer, state_b);
        }
    }
}
#[cfg(feature = "ssr")]
mod ssr {
    use async_str_iter::chain::Chain;

    use crate::ssr::SsrAttributes;

    use super::ChainAttributes;

    impl<A: SsrAttributes, B: SsrAttributes> SsrAttributes for ChainAttributes<A, B> {
        type IntoSsrAttributes = Chain<A::IntoSsrAttributes, B::IntoSsrAttributes>;

        fn into_ssr_attributes(self) -> Self::IntoSsrAttributes {
            Chain::new(self.0.into_ssr_attributes(), self.1.into_ssr_attributes())
        }
    }
}
