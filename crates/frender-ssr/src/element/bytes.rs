use std::{pin::Pin, task::Poll};

use frender_core::RenderState;

use crate::{
    bytes::{AsyncWritableBytes, IntoAsyncWritableBytes},
    SsrContext, WriterOrError,
};

pub struct State<B> {
    buf: B,
    is_writing: bool,
}

impl<B> Unpin for State<B> {}

impl<W: crate::AsyncWrite + Unpin, B: AsyncWritableBytes> RenderState<SsrContext<W>> for State<B> {
    fn unmount(self: Pin<&mut Self>) {}

    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut SsrContext<W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let State { buf, is_writing } = self.get_mut();

        if buf.as_ref().is_empty() {
            return Poll::Ready(());
        }

        let writer_or_error = match ctx.use_writer_and_mark_busy(is_writing) {
            Some(w) => w,
            // cx.waker() is not used
            // because a former RenderState must also be pending and used the waker
            None => return Poll::Pending,
        };

        match writer_or_error {
            WriterOrError::Writer(writer) => match buf.poll_write_bytes(Pin::new(writer), cx) {
                Poll::Ready(Ok(_)) => {}
                Poll::Ready(Err(error)) => {
                    *writer_or_error = WriterOrError::Error(error);
                }
                Poll::Pending => return Poll::Pending,
            },
            WriterOrError::Error(_) => {}
        }

        *is_writing = false;
        ctx.busy = false;
        Poll::Ready(())
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
            type State = $crate::element::bytes::State<<$buffer_ty as $crate::bytes::IntoAsyncWritableBytes>::Bytes>;

            fn initialize_render_state($self_ident, ctx: &mut $crate::SsrContext<W>) -> Self::State {
                $crate::element::bytes::UnsafeRawHtmlBytes($expr).initialize_render_state(ctx)
            }

            fn update_render_state(
                $self_ident,
                ctx: &mut $crate::SsrContext<W>,
                state: ::core::pin::Pin<&mut Self::State>,
            ) {
                $crate::element::bytes::UnsafeRawHtmlBytes($expr).update_render_state(ctx, state)
            }
        }
    )*};
}

impl<B: IntoAsyncWritableBytes, W: crate::AsyncWrite + ::core::marker::Unpin>
    ::frender_core::UpdateRenderState<crate::SsrContext<W>> for UnsafeRawHtmlBytes<B>
{
    type State = crate::element::bytes::State<<B as crate::bytes::IntoAsyncWritableBytes>::Bytes>;
    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        State {
            buf: <B as crate::bytes::IntoAsyncWritableBytes>::into_async_writable_bytes(self.0),
            is_writing: false,
        }
    }
    fn update_render_state(
        self,
        ctx: &mut crate::SsrContext<W>,
        state: ::core::pin::Pin<&mut Self::State>,
    ) {
        let state = state.get_mut();

        if state.is_writing && ctx.busy {
            ctx.busy = false;
            // TODO: warn that the writer has already be written with old elements but not done.
            eprintln!("the writer has already be written with old elements but not done");
        }

        *state = self.initialize_render_state(ctx)
    }
}
