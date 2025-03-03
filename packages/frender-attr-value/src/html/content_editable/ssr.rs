use frender_common::Empty;

use crate::{
    html::{bool_to_str, AttrKindOfContentEditable},
    known::KnownSsrStr,
    ssr::SsrAttrValue,
    AttrKindOfStr,
};

// TODO: optimize escaping
impl SsrAttrValue<AttrKindOfContentEditable> for bool {
    type HtmlAttributeValue = <&'static str as SsrAttrValue<AttrKindOfStr>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <&'static str as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(bool_to_str(
            this,
        ))
    }
}

impl SsrAttrValue<AttrKindOfContentEditable> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl<V: KnownSsrStr> SsrAttrValue<AttrKindOfContentEditable> for V {
    type HtmlAttributeValue = <Self as SsrAttrValue<AttrKindOfStr>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <Self as SsrAttrValue<AttrKindOfStr>>::maybe_into_html_attribute_value(this)
    }
}
