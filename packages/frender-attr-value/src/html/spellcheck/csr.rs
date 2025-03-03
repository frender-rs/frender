use crate::{
    csr::CsrAttrValue, html::Spellcheck, impl_csr_attr_value_for_unit_struct,
    impl_csr_attr_value_with_cache,
};

impl CsrAttrValue<Spellcheck> for Spellcheck {
    type State = bool;

    impl_csr_attr_value_with_cache!(
        kind![Spellcheck],
        set = |this| this,
        into_cache = this.0,
        eq = |this, cache| this.0 == *cache,
    );
}

impl CsrAttrValue<Spellcheck> for bool {
    type State = Self;

    impl_csr_attr_value_with_cache!(
        kind![Spellcheck],
        set = |this| Spellcheck(this),
        eq = Self::eq,
    );
}

use frender_common::Empty;

impl CsrAttrValue<Spellcheck> for Empty {
    impl_csr_attr_value_for_unit_struct!((Spellcheck::EMPTY) as Spellcheck);
}
