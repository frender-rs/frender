use frender_common::Empty;
use frender_reactive_value::value_kind::KindOfTempRef;

use crate::{
    csr::{
        cached_some::{AttrValueKindWithReactiveValueKind, CsrAttrValueCachedSome},
        CsrAttrValue, UpdateAttrValue,
    },
    html::{bool_to_str, AttrKindOfContentEditable},
    impl_csr_attr_value_for_unit_struct, impl_csr_attr_value_with_cache,
    known::KnownCsrStr,
    AttrKindOfStr,
};

impl CsrAttrValue<AttrKindOfContentEditable> for bool {
    type State = Self;

    impl_csr_attr_value_with_cache!(
        kind![AttrKindOfContentEditable],
        set = |this| bool_to_str(this),
        eq = Self::eq,
    );
}

impl CsrAttrValue<AttrKindOfContentEditable> for Empty {
    impl_csr_attr_value_for_unit_struct!(("") as AttrKindOfContentEditable);
}

struct UpdateStr<U: UpdateAttrValue<Kind = AttrKindOfContentEditable>>(U);

impl<U: UpdateAttrValue<Kind = AttrKindOfContentEditable>> UpdateAttrValue for UpdateStr<U> {
    type Kind = AttrKindOfStr;

    fn set(self, value: &str) {
        self.0.set(value)
    }

    fn remove(self) {
        self.0.remove()
    }
}

impl AttrValueKindWithReactiveValueKind for AttrKindOfContentEditable {
    type ReactiveValueKind = KindOfTempRef<str>;

    fn reactive_value_into_attr_value(
        value: <Self::ReactiveValueKind as frender_reactive_value::value_kind::ValueKind>::Value<
            '_,
        >,
    ) -> Self::AttrValue<'_> {
        value.0
    }
}

impl<V: KnownCsrStr> CsrAttrValueCachedSome<AttrKindOfContentEditable> for V {}
