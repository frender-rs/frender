use async_str_iter::ext::{collect::Collect, AsyncStrIteratorExt};

use crate::SsrElement;

pub type RenderToString<S> = Collect<S, String>;

pub trait SsrElementExt: SsrElement {
    fn render_to_string(self) -> RenderToString<Self::HtmlChildren>
    where
        Self: Sized,
    {
        self.into_html_children().collect()
    }
}

impl<E: SsrElement + ?Sized> SsrElementExt for E {}
