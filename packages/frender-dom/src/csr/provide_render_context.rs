use crate::csr::render::RenderWithContext;

pub trait ProvideRenderContext {
    type Renderer: ?Sized + RenderWithContext;

    // Should provide the same render context in multiple calls
    fn provide_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res;

    fn renderer_mut(&mut self) -> &mut Self::Renderer;
}

impl<P: ?Sized + ProvideRenderContext> ProvideRenderContext for &mut P {
    type Renderer = P::Renderer;

    fn provide_render_context<Res>(
        &mut self,
        f: impl FnOnce(&mut <Self::Renderer as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        P::provide_render_context(self, f)
    }

    fn renderer_mut(&mut self) -> &mut Self::Renderer {
        P::renderer_mut(self)
    }
}
