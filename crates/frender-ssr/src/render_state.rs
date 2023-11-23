use std::{io, pin::Pin, task::Poll};

use futures_io::AsyncWrite;

pub use SsrRenderState as RenderState;

pub trait SsrRenderState {
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

mod async_write_str {
    use std::{io, pin::Pin, task::Poll};

    pub trait AsyncWriteStr {
        fn poll_write_str(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            s: &str,
        ) -> Poll<io::Result<()>>;
    }

    fn poll_write_all<W: ?Sized + futures_io::AsyncWrite>(
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
        mut buf: &[u8],
    ) -> Poll<io::Result<()>> {
        let buf = &mut buf;
        while !buf.is_empty() {
            let n = crate::ready!(writer.as_mut().poll_write(cx, buf))?;
            let (_, rest) = std::mem::take(buf).split_at(n);
            *buf = rest;

            if n == 0 {
                return Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
        }

        Poll::Ready(Ok(()))
    }

    pub struct Utf8<W: ?Sized + futures_io::AsyncWrite>(pub W);

    impl<W: ?Sized + futures_io::AsyncWrite> AsyncWriteStr for Utf8<W> {
        fn poll_write_str(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            s: &str,
        ) -> Poll<io::Result<()>> {
            // SAFETY: pin project
            let writer = unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) };

            poll_write_all(writer, cx, s.as_bytes())
        }
    }

    pin_project_lite::pin_project!(
        pub struct Utf16<W: ?Sized> {
            pub encoded: Vec<u8>,
            #[pin]
            pub write: W,
        }
    );

    impl<W: ?Sized + futures_io::AsyncWrite> AsyncWriteStr for Utf16<W> {
        fn poll_write_str(
            self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            s: &str,
        ) -> Poll<io::Result<()>> {
            let this = self.project();

            for c in s.encode_utf16() {}

            todo!()
        }
    }
}
