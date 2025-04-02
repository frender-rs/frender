use crate::{AttrValue, AttrValueKind};

pub enum NeverAttrValue {}

impl<VK: AttrValueKind> crate::sealed::AttrValue<VK> for NeverAttrValue {}
impl<VK: AttrValueKind> AttrValue<VK> for NeverAttrValue {}

#[cfg(feature = "csr")]
mod csr {
    use crate::{
        csr::{CsrAttrValue, CsrAttrValueState},
        AttrValueKind,
    };

    use super::NeverAttrValue;

    pub enum State {}

    impl CsrAttrValueState for State {
        fn attribute_is_known_as_absent(&self) -> bool {
            match *self {}
        }
    }

    impl<VK: AttrValueKind> CsrAttrValue<VK> for NeverAttrValue {
        type State = State;

        fn render_init_on_absent_attribute(
            this: Self,
            _: impl crate::csr::UpdateAttrValue<Kind = VK>,
        ) -> Self::State {
            match this {}
        }

        fn render_init(this: Self, _: impl crate::csr::UpdateAttrValue<Kind = VK>) -> Self::State {
            match this {}
        }

        fn render_init_by_reusing_on_absent_attribute(
            this: Self,
            _: impl crate::csr::UpdateAttrValue<Kind = VK>,
            _: &mut Self::State,
        ) {
            match this {}
        }

        fn render_init_by_reusing(
            this: Self,
            _: impl crate::csr::UpdateAttrValue<Kind = VK>,
            _: &mut Self::State,
        ) {
            match this {}
        }

        fn render_update(
            this: Self,
            _: impl crate::csr::UpdateAttrValue<Kind = VK>,
            _: &mut Self::State,
        ) {
            match this {}
        }
    }
}
#[cfg(feature = "ssr")]
mod ssr {
    use crate::{ssr::SsrAttrValue, AttrValueKind};

    use super::NeverAttrValue;

    impl<VK: AttrValueKind> SsrAttrValue<VK> for NeverAttrValue {
        type HtmlAttributeValue = async_str_iter::never::Never;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            match this {}
        }
    }
}
