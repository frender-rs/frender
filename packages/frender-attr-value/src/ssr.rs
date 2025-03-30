use frender_ssr_html::assert::HtmlAttributeEqValueOrEmpty;

use crate::{AttrValueKind, IntoAttrValue};

/// A *html attribute value* is `=value`, `="value"`, `='value'` or empty.
pub trait SsrAttrValue<AK: AttrValueKind> {
    type HtmlAttributeValue: HtmlAttributeEqValueOrEmpty;

    /// `None` indicates this attributes is not present
    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue>;
}

impl<T: IntoAttrValue<AK>, AK: AttrValueKind> SsrAttrValue<AK> for T {
    type HtmlAttributeValue = <T::IntoAttrValue as SsrAttrValue<AK>>::HtmlAttributeValue;

    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue> {
        <T::IntoAttrValue as SsrAttrValue<AK>>::maybe_into_html_attribute_value(
            this.into_attr_value(),
        )
    }
}
