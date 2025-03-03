use crate::{ssr::SsrAttrValue, AttrValueKind};

impl<AK: AttrValueKind, V: SsrAttrValue<AK>> SsrAttrValue<AK> for Option<V> {
    type HtmlAttributeValue = V::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        this.and_then(V::maybe_into_html_attribute_value)
    }
}
