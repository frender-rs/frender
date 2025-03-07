use crate::{AttrValue, AttrValueKind};

/// Indicates an attribute is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Absent;

impl<VK: AttrValueKind> AttrValue<VK> for Absent {}

#[cfg(feature = "ssr")]
mod ssr {
    use crate::{ssr::SsrAttrValue, AttrValueKind};

    use super::Absent;

    impl<VK: AttrValueKind> SsrAttrValue<VK> for Absent {
        type HtmlAttributeValue = async_str_iter::never::Never;

        fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
            None
        }
    }
}

#[cfg(feature = "csr")]
mod csr {
    use crate::{
        csr::{CsrAttrValue, UpdateAttrValue},
        AttrValueKind,
    };

    use super::Absent;

    impl<VK: AttrValueKind> CsrAttrValue<VK> for Absent {
        type State = ();

        fn update_absent_attribute_value_into_state(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
        ) -> Self::State {
            // already absent
        }

        fn update_attribute_value_into_state(
            Self: Self,
            updater: impl UpdateAttrValue<Kind = VK>,
        ) -> Self::State {
            updater.remove()
        }

        fn can_skip_update(Self: &Self, (): &Self::State) -> bool {
            // It has been updated in self._into_state()
            // and there will be no further updates.
            true
        }

        fn update_attribute_value_with_state(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
            (): &mut Self::State,
        ) {
            // skip update
        }

        fn force_update_attribute_value_with_state(
            Self: Self,
            _: impl UpdateAttrValue<Kind = VK>,
            (): &mut Self::State,
        ) {
            // the attribute has already been made sure as absent in self._into_state()
        }

        fn attribute_is_known_as_absent((): &Self::State) -> bool {
            true
        }
    }
}
