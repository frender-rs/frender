use crate::{ssr::SsrAttrValue, AttrValueKind};

use super::OptionAttrValue;

impl<AK: AttrValueKind, V: SsrAttrValue<AK>> SsrAttrValue<AK> for OptionAttrValue<V> {
    type HtmlAttributeValue = V::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(Self(this): Self) -> Option<Self::HtmlAttributeValue> {
        this.and_then(V::maybe_into_html_attribute_value)
    }
}
