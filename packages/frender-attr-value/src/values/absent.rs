use crate::{AttrValue, AttrValueKind};

pub struct AbsentAttributeValue;

impl<VK: AttrValueKind> crate::sealed::AttrValue<VK> for AbsentAttributeValue {}
impl<VK: AttrValueKind> AttrValue<VK> for AbsentAttributeValue {}

#[cfg(feature = "ssr")]
mod ssr {
    use crate::{ssr::SsrAttrValue, AttrValueKind};

    use super::AbsentAttributeValue;

    impl<VK: AttrValueKind> SsrAttrValue<VK> for AbsentAttributeValue {
        type HtmlAttributeValue = async_str_iter::never::Never;

        fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
            None
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use crate::{
        csr::{CsrAttrValue, CsrAttrValueState, UpdateAttrValue},
        AttrValueKind,
    };

    use super::AbsentAttributeValue;

    pub struct State;

    impl CsrAttrValueState for State {
        fn attribute_is_known_as_absent(&self) -> bool {
            true
        }
    }

    impl<VK: AttrValueKind> CsrAttrValue<VK> for AbsentAttributeValue {
        type State = State;

        fn render_init_on_absent_attribute(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
        ) -> Self::State {
            // already absent
            State
        }

        fn render_init(Self: Self, updater: impl UpdateAttrValue<Kind = VK>) -> Self::State {
            updater.remove();
            State
        }

        fn render_init_by_reusing_on_absent_attribute(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
            self::State: &mut Self::State,
        ) {
            // the attribute is already absent
        }

        fn render_init_by_reusing(
            Self: Self,
            updater: impl UpdateAttrValue<Kind = VK>,
            self::State: &mut Self::State,
        ) {
            updater.remove()
        }

        fn render_update(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
            self::State: &mut Self::State,
        ) {
            // skip update
        }
    }
}
