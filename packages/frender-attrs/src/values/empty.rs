pub struct EmptyAttributes;

impl crate::sealed::Attributes for EmptyAttributes {}
impl crate::Attributes for EmptyAttributes {}

#[cfg(feature = "csr")]
mod csr {
    use super::EmptyAttributes;

    use crate::csr::{CsrAttributes, CsrAttributesStateUnmount};

    pub struct EmptyState;

    impl CsrAttributesStateUnmount for EmptyState {
        fn state_unmount(&mut self, _: &mut impl crate::csr::RenderAttributes) {}
    }

    impl CsrAttributes for EmptyAttributes {
        type State = EmptyState;

        fn render_init(self, _: &mut impl crate::csr::RenderAttributes) -> Self::State {
            EmptyState
        }

        fn render_update(
            self,
            _: &mut impl crate::csr::RenderAttributes,
            self::EmptyState: &mut Self::State,
        ) {
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use super::EmptyAttributes;

    use crate::ssr::SsrAttributes;

    impl SsrAttributes for EmptyAttributes {}
}
