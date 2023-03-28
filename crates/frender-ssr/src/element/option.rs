use std::pin::Pin;

use crate::{Element, RenderState};

impl<S: RenderState + Unpin> RenderState for Option<S> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.get_mut() {
            Some(s) => S::poll_render(Pin::new(s), writer, cx),
            None => std::task::Poll::Ready(Ok(())),
        }
    }
}

impl<E: Element> Element for Option<E>
where
    <E as Element>::SsrState: Unpin,
{
    type SsrState = Option<E::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        self.map(E::into_ssr_state)
    }
}
