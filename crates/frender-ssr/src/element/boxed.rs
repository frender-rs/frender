use crate::SsrElement;

impl<E: SsrElement> SsrElement for Box<E> {
    type HtmlChildren = E::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        E::into_html_children(*self)
    }
}
