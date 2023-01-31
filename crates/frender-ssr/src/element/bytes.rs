use std::{pin::Pin, task::Poll};

use frender_core::{RenderState, UpdateRenderState};

use crate::{
    bytes::{AsyncWritableBytes, IntoAsyncWritableBytes},
    WriterOrError,
};

pub struct StateInner<W, B> {
    writer_or_error: WriterOrError<W>,
    buf: B,
}

pub struct State<W, B> {
    inner: Option<StateInner<W, B>>,
}

impl<W, B> Unpin for State<W, B> {}

impl<W: crate::AsyncWrite + Unpin, B: AsyncWritableBytes> RenderState for State<W, B> {
    fn new_uninitialized() -> Self
    where
        Self: Sized,
    {
        Self { inner: None }
    }

    fn unmount(self: Pin<&mut Self>) {
        self.get_mut().inner = None;
    }

    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let inner = &mut self.get_mut().inner;
        match inner {
            Some(StateInner {
                writer_or_error,
                buf,
            }) => match writer_or_error {
                WriterOrError::Writer(writer) => match buf.poll_write_bytes(Pin::new(writer), cx) {
                    Poll::Ready(Ok(_)) => Poll::Ready(false),
                    Poll::Ready(Err(error)) => {
                        *writer_or_error = WriterOrError::Error(error);
                        Poll::Pending
                    }
                    Poll::Pending => Poll::Pending,
                },
                WriterOrError::Error(_) => Poll::Ready(false),
            },
            None => Poll::Ready(true),
        }
    }
}

pub struct UnsafeRawHtmlBytes<B: IntoAsyncWritableBytes>(pub B);

impl<W: crate::AsyncWrite + Unpin, B: IntoAsyncWritableBytes>
    UpdateRenderState<crate::SsrContext<W>> for UnsafeRawHtmlBytes<B>
{
    type State = State<W, B::Bytes>;

    fn update_render_state(
        self,
        ctx: &mut crate::SsrContext<W>,
        state: std::pin::Pin<&mut Self::State>,
    ) {
        let state = state.get_mut();

        let writer_or_error = ctx
            .writer_or_error
            .take()
            .expect("ssr context has no writer");

        state.inner = Some(StateInner {
            writer_or_error,
            buf: self.0.into_async_writable_bytes(),
        });
    }
}
