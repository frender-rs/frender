use std::pin::Pin;

use crate::{Element, IntoAsyncStrIterator};

impl<E: Element> Element for Option<E> {
    type HtmlChildren = crate::async_str::option::IterOption<E::HtmlChildren>;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.map(E::into_html_children).into_async_str_iterator()
    }
}
