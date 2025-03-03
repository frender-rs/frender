use crate::{AttrValue, AttrValueKind};

frender_common::impl_many!(
    impl<__> AttrValueKind
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type AttrValue<'a> = Self;
    }
);

frender_common::impl_many!(
    impl<__> AttrValue<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
    }
);

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
