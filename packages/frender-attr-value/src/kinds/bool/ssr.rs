use frender_common::Empty;

use crate::ssr::SsrAttrValue;

impl SsrAttrValue<bool> for Empty {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl SsrAttrValue<bool> for bool {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        this.then_some(async_str_iter::empty::Empty)
    }
}
