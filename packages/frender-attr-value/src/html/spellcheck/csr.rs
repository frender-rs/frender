use crate::{
    csr::CsrAttrValue, html::AttrKindOfSpellcheck, impl_csr_attr_value_for_unit_struct,
    impl_csr_attr_value_with_cache,
};

impl CsrAttrValue<AttrKindOfSpellcheck> for bool {
    type State = Self;

    impl_csr_attr_value_with_cache!(
        kind![AttrKindOfSpellcheck],
        set = |this| this,
        eq = Self::eq,
    );
}

use frender_common::Empty;

impl CsrAttrValue<AttrKindOfSpellcheck> for Empty {
    impl_csr_attr_value_for_unit_struct!((AttrKindOfSpellcheck::EMPTY) as AttrKindOfSpellcheck);
}

impl AttrKindOfSpellcheck {
    /// empty string or true, which indicates that the element should be, if possible, checked for spelling errors
    pub(crate) const EMPTY: bool = true;
}
