pub trait MapValueToSsrElement<V: ?Sized>: super::MapValueToElement<V> {
    type HtmlChildrenWithValue: frender_ssr::html::assert::HtmlChildren;

    fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildrenWithValue;
}
