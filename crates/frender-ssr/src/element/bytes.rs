use std::{pin::Pin, task::Poll};

use frender_core::RenderState;

use crate::{
    bytes::{AsyncWritableBytes, IntoAsyncWritableBytes},
    Ssr, SsrContext, WritingState,
};

pub struct State<B> {
    buf: B,
    writing_state: WritingState,
}

impl<B> Unpin for State<B> {}

impl<W: crate::AsyncWrite + ?Sized, B: AsyncWritableBytes> RenderState<Ssr<W>> for State<B> {
    fn unmount(self: Pin<&mut Self>) {
        panic!("ssr element can not be unmounted");
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut SsrContext<Pin<&mut W>>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        let State { buf, writing_state } = self.get_mut();
        ctx.map_writer(
            writing_state,
            |writer, cx| buf.poll_write_bytes(Pin::new(writer), cx),
            cx,
        )
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
            W: $crate::AsyncWrite + ?::core::marker::Sized> ::frender_core::UpdateRenderState<$crate::Ssr<W>
        > for $for_ty
        {
            type State = $crate::element::bytes::State<<$buffer_ty as $crate::bytes::IntoAsyncWritableBytes>::Bytes>;

            fn initialize_render_state(
                $self_ident,
                ctx: &mut $crate::SsrContext<::core::pin::Pin<&mut W>>,
            ) -> Self::State {
                $crate::element::bytes::UnsafeRawHtmlBytes::<$buffer_ty>($expr).initialize_render_state(ctx)
            }

            fn update_render_state(
                $self_ident,
                ctx: &mut $crate::SsrContext<::core::pin::Pin<&mut W>>,
                state: ::core::pin::Pin<&mut Self::State>,
            ) {
                $crate::element::bytes::UnsafeRawHtmlBytes::<$buffer_ty>($expr).update_render_state(ctx, state)
            }
        }
    )*};
}

impl<B: IntoAsyncWritableBytes, W: crate::AsyncWrite + ?Sized>
    ::frender_core::UpdateRenderState<crate::Ssr<W>> for UnsafeRawHtmlBytes<B>
{
    type State = crate::element::bytes::State<<B as crate::bytes::IntoAsyncWritableBytes>::Bytes>;
    fn initialize_render_state(self, ctx: &mut crate::SsrContext<Pin<&mut W>>) -> Self::State {
        State {
            buf: <B as crate::bytes::IntoAsyncWritableBytes>::into_async_writable_bytes(self.0),
            writing_state: Default::default(),
        }
    }
    fn update_render_state(
        self,
        ctx: &mut crate::SsrContext<Pin<&mut W>>,
        state: ::core::pin::Pin<&mut Self::State>,
    ) {
        panic!("ssr element can not be updated");
    }
}
