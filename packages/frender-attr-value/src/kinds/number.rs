use frender_common::impl_many;
use frender_reactive_value::value_kind::KindOfOwned;

use crate::{
    csr::cached_some::AttrValueKindWithReactiveValueKind, sealed::AttrValue as SealedAttrValue,
    values::cached_some::CachedSome, AttrValue, AttrValueKind, IntoAttrValue,
};

impl_many!(
    impl<__> AttrValueKind
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type AttrValue<'a> = Self;
    }
);

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

type CachedSomeSelf<T> = CachedSome<T, KindOfOwned<T>>;

impl_many!(
    impl<__> IntoAttrValue<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type IntoAttrValue = CachedSomeSelf<Self>;
        fn into_attr_value(self) -> Self::IntoAttrValue {
            CachedSome::new(self)
        }
    }
);

macro_rules! impl_for_cached_some {
    (
        impl $(<__>)? $Trait:ident<_>
        for each_of![$($for_ty:ty),+ $(,)?]
        $imp:tt
    ) => {$(
        impl $Trait<$for_ty> for CachedSomeSelf<$for_ty>
        $imp
    )+};
}

impl_for_cached_some!(
    impl<__> SealedAttrValue<_>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
    }
);

impl_for_cached_some!(
    impl<__> AttrValue<_>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
    }
);

#[cfg(feature = "ssr")]
mod ssr;
