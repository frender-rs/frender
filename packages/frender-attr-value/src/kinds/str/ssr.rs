use frender_ssr_html::attr_value::AttrEqValue;

use crate::{known::KnownStr, ssr::SsrAttrValue};

use super::{AttrKindOfStr, CachedSomeStr};

impl<S: KnownStr> SsrAttrValue<AttrKindOfStr> for CachedSomeStr<S> {
    type HtmlAttributeValue = AttrEqValue<S::SsrStrIntoAsyncStrIterator>;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        let this = this.0;
        Some(AttrEqValue::new(this.ssr_str_into_async_str_iterator()))
    }
}
