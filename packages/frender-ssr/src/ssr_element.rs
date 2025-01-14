pub trait SsrElement {
    type HtmlChildren: frender_ssr_html::assert::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren;
}
