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

            type IntoAsyncHtmlChunks = frender_ssr_html::encode::Encode<frender_ssr_html::escape_safe::Safe, <Self as frender_common::IntoAsyncStrIterator>::IntoAsyncStrIterator>;

            fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
                Self::IntoAsyncHtmlChunks::new(
                    frender_ssr_html::escape_safe::Safe,
                    frender_common::IntoAsyncStrIterator::into_async_str_iterator(self),
                )
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

    type IntoAsyncHtmlChunks = frender_common::async_str::never::Never;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
        todo!()
    }
}
