use async_str_iter::IntoAsyncStrIterator;

use crate::Element;

impl<E: Element> Element for Option<E> {
    type HtmlChildren = async_str_iter::option::IterOption<E::HtmlChildren>;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.map(E::into_html_children).into_async_str_iterator()
    }
}
