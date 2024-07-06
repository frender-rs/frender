pub trait SsrElement {
    type HtmlChildren: frender_ssr_html::assert::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren;
}

pub trait ToSsrElement {
    type ToSsrElement: SsrElement;
    fn to_ssr_element(&self) -> Self::ToSsrElement;
}

impl<E: Copy + SsrElement> ToSsrElement for E {
    type ToSsrElement = Self;

    fn to_ssr_element(&self) -> Self::ToSsrElement {
        *self
    }
}

impl ToSsrElement for str {
    type ToSsrElement = String;

    fn to_ssr_element(&self) -> Self::ToSsrElement {
        self.to_owned()
    }
}
