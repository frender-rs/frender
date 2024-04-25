use frender_ssr_html::char::IterCharStringEncodeSafe;

use crate::SsrElement;

impl SsrElement for char {
    type HtmlChildren = IterCharStringEncodeSafe;

    fn into_html_children(self) -> Self::HtmlChildren {
        IterCharStringEncodeSafe::new(self)
    }
}
