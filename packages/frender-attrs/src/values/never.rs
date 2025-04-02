use crate::Attributes;

pub enum NeverAttributes {}

impl crate::sealed::Attributes for NeverAttributes {}
impl Attributes for NeverAttributes {}

#[cfg(feature = "csr")]
mod csr {
    use crate::csr::{CsrAttributes, CsrAttributesStateUnmount, RenderAttributes};

    use super::NeverAttributes;

    pub enum State {}

    impl CsrAttributesStateUnmount for State {
        fn state_unmount(&mut self, _: &mut impl RenderAttributes) {
            match *self {}
        }
    }

    impl CsrAttributes for NeverAttributes {
        type State = State;

        fn render_init(self, _: &mut impl RenderAttributes) -> Self::State {
            match self {}
        }

        fn render_update(self, _: &mut impl RenderAttributes, _: &mut Self::State) {
            match self {}
        }
    }
}
#[cfg(feature = "ssr")]
mod ssr {
    use crate::ssr::SsrAttributes;

    use super::NeverAttributes;

    impl SsrAttributes for NeverAttributes {
        type IntoSsrAttributes = async_str_iter::never::Never;

        fn into_ssr_attributes(self) -> Self::IntoSsrAttributes {
            match self {}
        }
    }
}
