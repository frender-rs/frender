use frender_common::impl_many;
use frender_reactive_value::value_kind::KindOfOwned;

use crate::csr::cached_some::AttrValueKindWithReactiveValueKind;

impl_many!(
    impl<__> AttrValueKindWithReactiveValueKind<KindOfOwned<Self>>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        fn reactive_value_into_attr_value(
            value: <KindOfOwned<Self> as frender_reactive_value::value_kind::ValueKind>::Value<'_>,
        ) -> Self::AttrValue<'_> {
            value
        }
    }
);
