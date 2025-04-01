use frender_reactive_value::value_kind::KindOfOwned;

use crate::{
    csr::const_some::impl_csr_attr_value_for_const_some,
    csr::{cached_some::AttrValueKindWithReactiveValueKind, CsrAttrValue},
    html::AttrKindOfSpellcheck,
};

use super::EmptyAsSpellcheck;

impl AttrValueKindWithReactiveValueKind<KindOfOwned<bool>> for AttrKindOfSpellcheck {
    fn reactive_value_into_attr_value(
        value: <KindOfOwned<bool> as frender_reactive_value::value_kind::ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_> {
        value
    }
}

impl CsrAttrValue<AttrKindOfSpellcheck> for EmptyAsSpellcheck {
    impl_csr_attr_value_for_const_some!((AttrKindOfSpellcheck::EMPTY) as AttrKindOfSpellcheck);
}

impl AttrKindOfSpellcheck {
    /// empty string or true, which indicates that the element should be, if possible, checked for spelling errors
    pub(crate) const EMPTY: bool = true;
}
