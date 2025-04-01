use frender_reactive_value::{temp_ref::TempRef, value_kind::KindOfTempRef};

use crate::csr::cached_some::AttrValueKindWithReactiveValueKind;

use super::AttrKindOfStr;

impl AttrValueKindWithReactiveValueKind<KindOfTempRef<str>> for AttrKindOfStr {
    fn reactive_value_into_attr_value(
        TempRef(value): <KindOfTempRef<str> as frender_reactive_value::value_kind::ValueKind>::Value<'_>,
    ) -> Self::AttrValue<'_> {
        value
    }
}
