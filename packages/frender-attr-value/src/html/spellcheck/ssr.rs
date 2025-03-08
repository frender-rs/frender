use frender_common::Empty;

use crate::{html::bool_to_str, ssr::SsrAttrValue, AttrKindOfStr};

use super::AttrKindOfSpellcheck;

impl SsrAttrValue<AttrKindOfSpellcheck> for bool {
    type HtmlAttributeValue = <&'static str as SsrAttrValue<AttrKindOfStr>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <&'static str as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(bool_to_str(
            this,
        ))
    }
}

impl SsrAttrValue<AttrKindOfSpellcheck> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}
