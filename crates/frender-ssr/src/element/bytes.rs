use std::{pin::Pin, task::Poll};

use crate::{
    bytes::{AsyncWritableBytes, IntoAsyncWritableBytes},
    Element, RenderState,
};

pub struct State<B> {
    buf: B,
}

impl<B> Unpin for State<B> {}

impl<B: AsyncWritableBytes> RenderState for State<B> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        let State { buf } = self.get_mut();
        buf.poll_write_bytes(writer, cx)
    }
}

pub struct UnsafeRawHtmlBytes<B: IntoAsyncWritableBytes>(pub B);

#[macro_export]
macro_rules! impl_ssr_for_bytes {
    ($(
        const _: fn($for_ty:ty) -> $buffer_ty:ty
        = |
            $self_ident:ident
            $(, generics: __![$($generic:tt)+])?
        | $expr:expr
    );* $(;)?) => {$(
        impl<
            $($($generic)+ ,)?
        > $crate::Element for $for_ty
        {
            type SsrState = $crate::element::bytes::State<<$buffer_ty as $crate::bytes::IntoAsyncWritableBytes>::Bytes>;

            fn into_ssr_state($self_ident) -> Self::SsrState {
                $crate::element::bytes::UnsafeRawHtmlBytes::<$buffer_ty>($expr).into_ssr_state()
            }

            type IntoIterHtmlChunk = Option<&'static str>;

            fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
                todo!()
            }
        }
    )*};
}

impl<B: IntoAsyncWritableBytes> Element for UnsafeRawHtmlBytes<B> {
    type SsrState = State<<B as crate::bytes::IntoAsyncWritableBytes>::Bytes>;

    fn into_ssr_state(self) -> Self::SsrState {
        State {
            buf: <B as crate::bytes::IntoAsyncWritableBytes>::into_async_writable_bytes(self.0),
        }
    }

    type IntoIterHtmlChunk = Option<&'static str>;

    fn into_iter_html_chunk(self) -> Self::IntoIterHtmlChunk {
        todo!()
    }
}
