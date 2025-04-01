use crate::ssr::SsrAttrValue;

use super::{BoolAsAttrValue, EmptyAsTrue};

impl SsrAttrValue<bool> for EmptyAsTrue {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self: Self) -> Option<Self::HtmlAttributeValue> {
        Some(async_str_iter::empty::Empty)
    }
}

impl SsrAttrValue<bool> for BoolAsAttrValue {
    type HtmlAttributeValue = async_str_iter::empty::Empty;

    fn maybe_into_html_attribute_value(Self(this): Self) -> Option<Self::HtmlAttributeValue> {
        this.then_some(async_str_iter::empty::Empty)
    }
}
