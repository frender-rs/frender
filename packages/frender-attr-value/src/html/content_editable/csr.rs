use frender_reactive_value::value_kind::{KindOfOwned, KindOfTempRef};

use crate::{
    csr::{
        cached_some::AttrValueKindWithReactiveValueKind,
        const_some::impl_csr_attr_value_for_const_some, CsrAttrValue, UpdateAttrValue,
    },
    html::{bool_to_str, AttrKindOfContentEditable},
    AttrKindOfStr,
};

use super::EmptyAsContentEditable;

impl CsrAttrValue<AttrKindOfContentEditable> for EmptyAsContentEditable {
    impl_csr_attr_value_for_const_some!(("") as AttrKindOfContentEditable);
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

impl AttrValueKindWithReactiveValueKind<KindOfTempRef<str>> for AttrKindOfContentEditable {
    fn reactive_value_into_attr_value(
        value: <KindOfTempRef<str> as frender_reactive_value::value_kind::ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_> {
        value.0
    }
}

impl AttrValueKindWithReactiveValueKind<KindOfOwned<bool>> for AttrKindOfContentEditable {
    fn reactive_value_into_attr_value(
        value: <KindOfOwned<bool> as frender_reactive_value::value_kind::ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_> {
        bool_to_str(value)
    }
}
