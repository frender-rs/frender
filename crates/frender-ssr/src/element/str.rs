impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for Cow<'_, str> {
    type State = ssr::State<W>;

    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        match self {
            Cow::Borrowed(data) => data.update_render_state(ctx, state),
            Cow::Owned(data) => data.update_render_state(ctx, state),
        }
    }
}

impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for &str {
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state
            .get_mut()
            .update_render_state_with_str(self.to_owned(), ctx)
    }
}

impl<W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>> for String {
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_render_state_with_str(self, ctx)
    }
}

impl<S: Into<Cow<'static, str>>, W: AsyncWrite + Unpin> UpdateRenderState<SsrContext<W>>
    for StaticText<S>
{
    type State = ssr::State<W>;

    #[inline]
    fn update_render_state(self, ctx: &mut SsrContext<W>, state: std::pin::Pin<&mut Self::State>) {
        state.get_mut().update_render_state_with_str(self.0, ctx);
    }
}
