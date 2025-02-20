use frender_dom::csr::{render::RenderWithContext, ProvideMutMounted, UiHandle, UnmountedUiHandle};

use super::ElementProxyAttrs;

impl<E: UnmountedUiHandle<R>, R: ?Sized> UnmountedUiHandle<R> for ElementProxyAttrs<E> {
    type Mounted = ElementProxyAttrs<E::Mounted>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: RenderWithContext,
    {
        ElementProxyAttrs(self.0.mount(render_context))
    }
}

impl<E: ProvideMutMounted<R>, R: ?Sized> ProvideMutMounted<R> for ElementProxyAttrs<E> {
    fn provide_mut_mounted<Out>(&mut self, renderer: &mut R, f: impl FnOnce(&mut R, &mut Self::Mounted) -> Out) -> Out {
        self.0.provide_mut_mounted(
            //
            renderer,
            |renderer, e| f(renderer, ElementProxyAttrs::<E::Mounted>::ref_cast_mut(e)),
        )
    }
}

impl<E: UiHandle<R>, R: ?Sized> UiHandle<R> for ElementProxyAttrs<E> {
    type Unmounted = ElementProxyAttrs<E::Unmounted>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        ElementProxyAttrs(self.0.unmount(renderer))
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.reposition(render_context)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.check_and_move_cursor(render_context)
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: RenderWithContext,
    {
        self.0.assert_cursor_is_at_self(render_context)
    }
}
