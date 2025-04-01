use frender_ssr_html::assert::HtmlAttributeEqValueOrEmpty;

use crate::AttrValueKind;

/// A *html attribute value* is `=value`, `="value"`, `='value'` or empty.
pub trait SsrAttrValue<AK: AttrValueKind> {
    type HtmlAttributeValue: HtmlAttributeEqValueOrEmpty;

    /// `None` indicates this attributes is not present
    fn maybe_into_html_attribute_value(this: Self) -> Option<Self::HtmlAttributeValue>;
}
