use crate::{html::bool_to_str, ssr::SsrAttrValue, AttrKindOfStr, IntoAttrValue};

use super::{AttrKindOfSpellcheck, CachedSomeBool, EmptyAsSpellcheck};

impl SsrAttrValue<AttrKindOfSpellcheck> for CachedSomeBool {
    type HtmlAttributeValue =
        <<&'static str as IntoAttrValue<AttrKindOfStr>>::IntoAttrValue as SsrAttrValue<
            AttrKindOfStr,
        >>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        let this = this.0;
        let this = bool_to_str(this);
        let this = <&'static str as IntoAttrValue<AttrKindOfStr>>::into_attr_value(this);
        <_ as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(this)
    }
}

impl SsrAttrValue<AttrKindOfSpellcheck> for EmptyAsSpellcheck {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}
