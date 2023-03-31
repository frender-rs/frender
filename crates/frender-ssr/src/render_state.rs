use std::{io, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

pub trait RenderState {
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>>;
}

impl<S: ?Sized + RenderState + Unpin> RenderState for &mut S {
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        S::poll_render(Pin::new(self.get_mut()), writer, cx)
    }
}

impl<S: ?Sized + RenderState + Unpin> RenderState for Box<S> {
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        S::poll_render(Pin::new(self.get_mut()), writer, cx)
    }
}

impl<P> RenderState for Pin<P>
where
    P: std::ops::DerefMut,
    P::Target: RenderState,
{
    fn poll_render<W: AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<io::Result<()>> {
        P::Target::poll_render(frender_common::utils::pin_as_deref_mut(self), writer, cx)
    }
}
