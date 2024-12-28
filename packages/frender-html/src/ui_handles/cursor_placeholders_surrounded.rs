use frender_dom::{
    behaviors::{NodeRenderSelf, NodeWithRenderContextAfterSelf},
    render::{RenderContext, RenderWithContext},
    ui_handle::{UiHandle, UnmountedUiHandle},
};

pub struct CursorPlaceholdersSurrounded<C, UH> {
    cursor_placeholders: [C; 2],
    ui_handle: UH,
}

impl<C, UH> CursorPlaceholdersSurrounded<C, UH> {
    pub fn surrounded_mut(&mut self) -> &mut UH {
        &mut self.ui_handle
    }

    pub fn surround<R: ?Sized + RenderWithContext>(
        //
        render_context: &mut R::RenderContext<'_>,
        f: impl FnOnce(&mut R::RenderContext<'_>) -> UH,
    ) -> Self
    where
        C: NodeRenderSelf<R>,
    {
        let a = C::render_self(render_context);
        let ui_handle = f(render_context);
        let b = C::render_self(render_context);
        Self {
            //
            cursor_placeholders: [a, b],
            ui_handle,
        }
    }

    pub fn surround_and_output<R: ?Sized + RenderWithContext, Out>(
        //
        render_context: &mut R::RenderContext<'_>,
        f: impl FnOnce(&mut R::RenderContext<'_>) -> (UH, Out),
    ) -> (Self, Out)
    where
        C: NodeRenderSelf<R>,
    {
        let a = C::render_self(render_context);
        let (ui_handle, out) = f(render_context);
        let b = C::render_self(render_context);
        (
            Self {
                //
                cursor_placeholders: [a, b],
                ui_handle,
            },
            out,
        )
    }

    pub fn map_mut_surrounded_with_render_context<R: ?Sized + RenderWithContext, Out>(
        //
        &mut self,
        render_context: &mut R::RenderContext<'_>,
        f: impl FnOnce(&mut UH, &mut R::RenderContext<'_>) -> Out,
    ) -> Out
    where
        C: UiHandle<R>,
    {
        let Self { cursor_placeholders: [a, b], ui_handle } = self;
        a.check_and_move_cursor(render_context);
        let out = f(ui_handle, render_context);
        b.check_and_move_cursor(render_context);
        out
    }

    /// `f` must use the render_context correctly (move the cursor just before the ending cursor placeholder)
    /// or this method panics.
    pub fn use_surrounded_render_context<R: ?Sized + RenderWithContext, Out>(
        //
        &mut self,
        renderer: &mut R,
        f: impl FnOnce(&mut UH, &mut R::RenderContext<'_>) -> Out,
    ) -> Out
    where
        C: UiHandle<R> + NodeWithRenderContextAfterSelf<R>,
    {
        let Self { cursor_placeholders: [a, b], ui_handle } = self;

        let out = a.with_render_context_after_self(renderer, |render_context| {
            let out = f(ui_handle, render_context);
            b.assert_cursor_is_at_self(render_context);
            out
        });

        out
    }
}

impl<C: UnmountedUiHandle<R>, UH: UnmountedUiHandle<R>, R: ?Sized> UnmountedUiHandle<R> for CursorPlaceholdersSurrounded<C, UH> {
    type Mounted = CursorPlaceholdersSurrounded<C::Mounted, UH::Mounted>;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: frender_dom::render::RenderWithContext,
    {
        let Self {
            cursor_placeholders: [start, end],
            ui_handle,
        } = self;

        let start = start.mount(render_context);
        let ui_handle = ui_handle.mount(render_context);
        let end = end.mount(render_context);
        CursorPlaceholdersSurrounded {
            cursor_placeholders: [start, end],
            ui_handle,
        }
    }
}

impl<C: UiHandle<R>, UH: UiHandle<R>, R: ?Sized> UiHandle<R> for CursorPlaceholdersSurrounded<C, UH> {
    type Unmounted = CursorPlaceholdersSurrounded<C::Unmounted, UH::Unmounted>;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        let Self {
            cursor_placeholders: [start, end],
            ui_handle,
        } = self;
        let start = start.unmount(renderer);
        let ui_handle = ui_handle.unmount(renderer);
        let end = end.unmount(renderer);
        CursorPlaceholdersSurrounded {
            cursor_placeholders: [start, end],
            ui_handle,
        }
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        let Self {
            cursor_placeholders: [start, end],
            ui_handle,
        } = self;
        start.reposition(render_context);
        ui_handle.reposition(render_context);
        end.reposition(render_context);
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        let Self {
            cursor_placeholders: [start, end],
            #[cfg(debug_assertions)]
            ui_handle,
            #[cfg(not(debug_assertions))]
                ui_handle: _, // Inner ui handles are skipped
        } = self;
        start.check_and_move_cursor(render_context);

        #[cfg(debug_assertions)]
        ui_handle.check_and_move_cursor(render_context);

        end.check_and_move_cursor(render_context);
    }

    fn assert_cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: frender_dom::render::RenderWithContext,
    {
        self.cursor_placeholders[0].assert_cursor_is_at_self(render_context)
    }
}
