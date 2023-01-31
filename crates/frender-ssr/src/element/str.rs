use std::{borrow::Cow, pin::Pin};

use frender_core::{StaticText, UpdateRenderState};

use crate::bytes::{CowSlicedBytes, SlicedBytes};

use super::bytes::{State, UnsafeRawHtmlBytes};

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for String {
    type State = State<W, SlicedBytes>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let s = match html_escape::encode_safe(&self) {
            Cow::Borrowed(_) => self,
            Cow::Owned(s) => s,
        };

        UnsafeRawHtmlBytes(s).update_render_state(ctx, state)
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for &str {
    type State = State<W, SlicedBytes>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let s = match html_escape::encode_safe(self) {
            Cow::Borrowed(s) => s.into(),
            Cow::Owned(s) => s,
        };

        UnsafeRawHtmlBytes(s).update_render_state(ctx, state)
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for Cow<'_, str> {
    type State = State<W, SlicedBytes>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        match self {
            Cow::Borrowed(s) => s.update_render_state(ctx, state),
            Cow::Owned(s) => s.update_render_state(ctx, state),
        }
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>>
    for StaticText<&'static str>
{
    type State = State<W, CowSlicedBytes<'static>>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let s = match html_escape::encode_safe(self.0) {
            Cow::Borrowed(s) => Cow::Borrowed(s),
            Cow::Owned(s) => Cow::Owned(s),
        };

        UnsafeRawHtmlBytes(s).update_render_state(ctx, state)
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for StaticText<String> {
    type State = State<W, SlicedBytes>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        self.0.update_render_state(ctx, state)
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>>
    for StaticText<Cow<'static, str>>
{
    type State = State<W, CowSlicedBytes<'static>>;

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        let s = match html_escape::encode_safe(&self.0) {
            Cow::Borrowed(_) => self.0,
            Cow::Owned(s) => Cow::Owned(s),
        };

        UnsafeRawHtmlBytes(s).update_render_state(ctx, state)
    }
}
