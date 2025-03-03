use frender_ssr_html::attr_value::AttrEqValue;

use crate::{known::KnownSsrStr, ssr::SsrAttrValue};

use super::AttrKindOfStr;

impl<S: KnownSsrStr> SsrAttrValue<AttrKindOfStr> for S {
    type HtmlAttributeValue = AttrEqValue<S::SsrStrIntoAsyncStrIterator>;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        Some(AttrEqValue::new(this.ssr_str_into_async_str_iterator()))
    }
}
