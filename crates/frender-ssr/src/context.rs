use std::{io, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

pub trait RenderState {
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>>;
}

pub trait Element {
    type SsrState: RenderState;

    fn into_ssr_state(self) -> Self::SsrState;
}
