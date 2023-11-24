use crate::RenderState;

pub use SsrElement as Element;

pub trait SsrElement {
    type SsrState: RenderState;

    fn into_ssr_state(self) -> Self::SsrState;

    type IntoAsyncHtmlChunks: crate::AsyncStrIterator;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks;
}
