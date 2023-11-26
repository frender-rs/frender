use std::pin::Pin;

use crate::{Element, IntoAsyncStrIterator, RenderState};

impl<S: RenderState> RenderState for Option<S> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if let Some(s) = self.as_pin_mut() {
            S::poll_render(s, writer, cx)
        } else {
            std::task::Poll::Ready(Ok(()))
        }
    }
}

impl<E: Element> Element for Option<E> {
    type SsrState = Option<E::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        self.map(E::into_ssr_state)
    }

    type IntoAsyncHtmlChunks = crate::async_str::option::IterOption<E::IntoAsyncHtmlChunks>;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
        self.map(E::into_async_html_chunks)
            .into_async_str_iterator()
    }
}
