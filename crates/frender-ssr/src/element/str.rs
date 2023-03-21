use std::{borrow::Cow, pin::Pin};

use frender_core::{StaticText, UpdateRenderState};

use crate::{bytes::SlicedBytes, impl_ssr_for_bytes};

use super::bytes::State;

impl_ssr_for_bytes! {
    String => String |self| match html_escape::encode_safe(&self) {
        Cow::Borrowed(_) => self,
        Cow::Owned(s) => s,
    };
    &str => String   |self| match html_escape::encode_safe(self) {
        Cow::Borrowed(s) => s.into(),
        Cow::Owned(s) => s,
    };
    StaticText<&'static str> => Cow<'static, str> |self| match html_escape::encode_safe(self.0) {
        Cow::Borrowed(s) => Cow::Borrowed(s),
        Cow::Owned(s) => Cow::Owned(s),
    };
    StaticText<Cow<'static, str>> => Cow<'static, str> |self| match html_escape::encode_safe(&self.0) {
        Cow::Borrowed(_) => self.0,
        Cow::Owned(s) => Cow::Owned(s),
    };
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for Cow<'_, str> {
    type State = State<SlicedBytes>;

    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        match self {
            Cow::Borrowed(s) => s.initialize_render_state(ctx),
            Cow::Owned(s) => s.initialize_render_state(ctx),
        }
    }

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        match self {
            Cow::Borrowed(s) => s.update_render_state(ctx, state),
            Cow::Owned(s) => s.update_render_state(ctx, state),
        }
    }
}

impl<W: crate::AsyncWrite + Unpin> UpdateRenderState<crate::SsrContext<W>> for StaticText<String> {
    type State = State<SlicedBytes>;

    fn initialize_render_state(self, ctx: &mut crate::SsrContext<W>) -> Self::State {
        self.0.initialize_render_state(ctx)
    }

    fn update_render_state(self, ctx: &mut crate::SsrContext<W>, state: Pin<&mut Self::State>) {
        self.0.update_render_state(ctx, state)
    }
}
