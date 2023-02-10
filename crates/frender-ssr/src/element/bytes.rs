use std::{pin::Pin, task::Poll};

use frender_core::RenderState;

use crate::{
    bytes::{AsyncWritableBytes, IntoAsyncWritableBytes},
    WriterOrError,
};

pub struct State<W, B> {
    writer_or_error: WriterOrError<W>,
    buf: B,
}

impl<W, B> Unpin for State<W, B> {}

impl<W: crate::AsyncWrite + Unpin, B: AsyncWritableBytes> RenderState for State<W, B> {
    fn unmount(self: Pin<&mut Self>) {}

    fn poll_reactive(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<bool> {
        let State {
            writer_or_error,
            buf,
        } = self.get_mut();
        match writer_or_error {
            WriterOrError::Writer(writer) => match buf.poll_write_bytes(Pin::new(writer), cx) {
                Poll::Ready(Ok(_)) => Poll::Ready(false),
                Poll::Ready(Err(error)) => {
                    *writer_or_error = WriterOrError::Error(error);
                    Poll::Pending
                }
                Poll::Pending => Poll::Pending,
            },
            WriterOrError::Error(_) => Poll::Ready(false),
        }
    }
}

pub struct UnsafeRawHtmlBytes<B: IntoAsyncWritableBytes>(pub B);

#[macro_export]
macro_rules! impl_ssr_for_bytes {
    ($(
        $(@[$($generic:tt)+])?
        $for_ty:ty => $buffer_ty:ty |$self_ident:ident| $expr:expr
    );* $(;)?) => {$(
        impl<
            $($($generic)+ ,)?
            W: $crate::AsyncWrite + ::core::marker::Unpin> ::frender_core::UpdateRenderState<$crate::SsrContext<W>
        > for $for_ty
        {
            type State = $crate::element::bytes::State<W, <$buffer_ty as $crate::bytes::IntoAsyncWritableBytes>::Bytes>;

            fn initialize_render_state($self_ident, ctx: &mut $crate::SsrContext<W>) -> Self::State {
                $crate::element::bytes::UnsafeRawHtmlBytes($expr).initialize_render_state(ctx)
            }

            fn update_render_state(
                self,
                ctx: &mut $crate::SsrContext<W>,
                state: ::core::pin::Pin<&mut Self::State>,
            ) {
                *state.get_mut() = self.initialize_render_state(ctx)
            }
        }
    )*};
}

impl<B: IntoAsyncWritableBytes, W: crate::AsyncWrite + ::core::marker::Unpin>
    ::frender_core::UpdateRenderState<crate::SsrContext<W>> for UnsafeRawHtmlBytes<B>
{
    type State =
        crate::element::bytes::State<W, <B as crate::bytes::IntoAsyncWritableBytes>::Bytes>;
    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        State {
            writer_or_error: ctx
                .writer_or_error
                .take()
                .expect("ssr context has no writer"),
            buf: <B as crate::bytes::IntoAsyncWritableBytes>::into_async_writable_bytes(self.0),
        }
    }
    fn update_render_state(
        self,
        ctx: &mut crate::SsrContext<W>,
        state: ::core::pin::Pin<&mut Self::State>,
    ) {
        *state.get_mut() = self.initialize_render_state(ctx)
    }
}
