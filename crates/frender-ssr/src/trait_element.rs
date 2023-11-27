use crate::RenderState;

pub use SsrElement as Element;

pub trait SsrElement {
    type SsrState: RenderState;

    fn into_ssr_state(self) -> Self::SsrState;

    type HtmlChildren: frender_ssr_html::assert::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren;
}
