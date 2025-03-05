use hooks::ShareValue;

use super::{SignalIntoElement, bound::ssr::MapValueToSsrElement};

impl<S: ShareValue, F> frender_ssr::SsrElement for SignalIntoElement<S, F>
where
    F: MapValueToSsrElement<S::Value>,
{
    type HtmlChildren = F::HtmlChildrenWithValue;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.map(|s| self.1.into_html_children_with_value(s))
    }
}
