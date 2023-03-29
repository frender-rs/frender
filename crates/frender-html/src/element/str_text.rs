use std::borrow::Cow;

use frender_core::UpdateRenderState;
use frender_csr::Dom;
use frender_ssr::{AsyncWrite, SsrContext};

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::{borrow::Cow, pin::Pin, task::Poll};

    use frender_core::RenderState;
    use frender_ssr::{AsyncWrite, SsrContext, SsrWriter};

    struct StateInner<W: AsyncWrite + Unpin> {
        writer: SsrWriter<W>,
        owned_buf: Cow<'static, [u8]>,
        written: usize,
    }

    pub struct State<W: AsyncWrite + Unpin>(Option<StateInner<W>>);

    impl<W: AsyncWrite + Unpin> State<W> {
        #[inline]
        pub(super) fn update_render_state_with_str(
            &mut self,
            buf: impl Into<Cow<'static, str>>,
            ctx: &mut SsrContext<W>,
        ) {
            self.0 = ctx.writer.take().map(|writer| {
                let buf: Cow<str> = buf.into();
                let escaped = html_escape::encode_safe(&buf);

                let buf = match escaped {
                    Cow::Borrowed(_) => buf,
                    Cow::Owned(escaped) => Cow::Owned(escaped),
                };

                let owned_buf = match buf {
                    Cow::Borrowed(s) => Cow::Borrowed(s.as_bytes()),
                    Cow::Owned(s) => Cow::Owned(s.into_bytes()),
                };

                StateInner {
                    writer,
                    owned_buf,
                    written: 0,
                }
            });
        }
    }

    fn poll_write_all<W: AsyncWrite>(
        mut writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
        written: &mut usize,
    ) -> Poll<std::io::Result<()>> {
        while *written < buf.len() {
            let buf = &buf[*written..];
            let n = futures_lite::ready!(writer.as_mut().poll_write(cx, buf))?;

            if n == 0 {
                return Poll::Ready(Err(std::io::ErrorKind::WriteZero.into()));
            }

            *written += n;
        }

        Poll::Ready(Ok(()))
    }

    impl<W: AsyncWrite + Unpin> RenderState for State<W> {
        fn new_uninitialized() -> Self {
            Self(None)
        }

        fn unmount(self: std::pin::Pin<&mut Self>) {
            self.get_mut().0 = None;
        }

        /// The implementation is from [`futures_lite::io::WriteAllFuture`].
        fn poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let this = &mut self.get_mut().0;
            if let Some(StateInner {
                writer,
                owned_buf,
                written,
            }) = this
            {
                if writer.error.is_none() {
                    if let Err(err) = futures_lite::ready!(poll_write_all(
                        Pin::new(&mut writer.writer),
                        cx,
                        owned_buf,
                        written
                    )) {
                        writer.error = Some(err)
                    }
                }
            }

            Poll::Ready(false)
        }
    }
}
