use frender_ssr_html::attr_value::AttrEqScalar;

use crate::ssr::SsrAttrValue;

use super::CachedSomeSelf;

impl_for_cached_some!(
    impl<__> SsrAttrValue<_>
        for each_of![i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64]
    {
        type HtmlAttributeValue = AttrEqScalar;

        fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
            let this = this.0;
            Some(AttrEqScalar::new(From::from(this)))
        }
    }
);
