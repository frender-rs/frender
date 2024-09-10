mod kind {
    use crate::csr::ValueKind;

    frender_common::impl_many!(
        impl<__> ValueKind
            for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
        {
            type Value<'a> = Self;
        }
    );
}

mod ssr {
    use crate::ssr::SsrAttrValue;

    // TODO: optimize unneeded escaping
    frender_common::impl_many!(
        impl<__> SsrAttrValue<Self>
            for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
        {
            type HtmlAttributeValue = <String as SsrAttrValue<str>>::HtmlAttributeValue;

            fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
                <String as SsrAttrValue<str>>::maybe_into_html_attribute_value(this.to_string())
            }
        }
    );
}

mod csr {
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
}
