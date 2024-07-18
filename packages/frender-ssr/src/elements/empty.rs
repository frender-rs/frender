use frender_common::Empty;

use crate::SsrElement;

impl SsrElement for Empty {
    type HtmlChildren = async_str_iter::empty::Empty;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::empty::Empty
    }
}
