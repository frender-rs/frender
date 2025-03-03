use crate::{csr::CsrAttrValue, impl_csr_attr_value_with_cache};

frender_common::impl_many!(
    impl<__> CsrAttrValue<Self>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type State = Self;

        impl_csr_attr_value_with_cache!(
            //
            kind![Self],
            set = |this| this,
            eq = Self::eq,
        );
    }
);
