pub trait RenderWithContext {
    type RenderContext<'a>: ?Sized + RenderContext<Renderer = Self>;
}

pub trait RenderContext {
    type Renderer: ?Sized + RenderWithContext;

    // TODO: remove
    fn map_mut_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res;

    fn renderer_mut(&mut self) -> &mut Self::Renderer;
    fn log_cursor(&mut self);
    fn mark_cursor_skipped(&mut self);
}
