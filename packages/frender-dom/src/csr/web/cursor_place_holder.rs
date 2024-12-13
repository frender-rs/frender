use crate::{
    behaviors,
    render::RenderWithContext,
    ui_handle::{UiHandle, UnmountedUiHandle},
};

use super::Renderer;

#[derive(Debug)]
pub struct CursorPlaceholder(web_sys::Comment);

impl<R: ?Sized + Renderer> UnmountedUiHandle<R> for CursorPlaceholder {
    type Mounted = Self;

    fn mount(self, render_context: &mut <R>::RenderContext<'_>) -> Self::Mounted
    where
        R: crate::render::RenderWithContext,
    {
        R::mount_node(render_context, &self.0);
        self
    }
}

impl<R: ?Sized + Renderer> UiHandle<R> for CursorPlaceholder {
    type Unmounted = Self;

    fn unmount(self, renderer: &mut R) -> Self::Unmounted {
        renderer.remove_node(&self.0);
        self
    }

    fn reposition(&mut self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        R::reposition_node(render_context, &self.0)
    }

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        R::check_and_move_cursor_after_node(render_context, &self.0)
    }

    fn assert_cursor_if_at_self(&self, render_context: &<R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        R::assert_cursor_if_at_node(render_context, &self.0)
    }
}

impl<R: ?Sized + Renderer> behaviors::Node<R> for CursorPlaceholder {
    fn log_self(&self, _: &mut R) {
        web_sys::console::log_2(&"CursorPlaceholder".into(), &self.0);
    }

    fn readd_self(&mut self, render_context: &mut <R>::RenderContext<'_>, force_reposition: bool)
    where
        R: crate::render::RenderWithContext,
    {
        R::readd_node(render_context, &mut self.0, force_reposition)
    }

    fn check_and_move_cursor_after_self(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        R::check_and_move_cursor_after_node(render_context, &self.0)
    }

    fn cursor_is_at_self(&self, render_context: &<R>::RenderContext<'_>) -> bool
    where
        R: crate::render::RenderWithContext,
    {
        R::cursor_is_at_node(render_context, &self.0)
    }

    fn remove_self(&mut self, renderer: &mut R) {
        renderer.remove_node(&self.0)
    }
}

impl<R: ?Sized + Renderer> behaviors::NodeRenderSelf<R> for CursorPlaceholder {
    fn render_self(
        render_context: &mut <R as crate::render::RenderWithContext>::RenderContext<'_>,
    ) -> Self {
        let mut node = render_context.renderer.document().create_comment("");
        R::readd_node(render_context, &mut node, true);
        Self(node)
    }
}

impl<R: ?Sized + Renderer> behaviors::NodeWithRenderContextAfterSelf<R> for CursorPlaceholder {
    fn with_render_context_after_self<Res>(
        &mut self,
        renderer: &mut R,
        f: impl FnOnce(&mut <R as RenderWithContext>::RenderContext<'_>) -> Res,
    ) -> Res {
        f(&mut super::RenderContext {
            renderer,
            cursor: &mut super::Cursor::after(std::borrow::Cow::Borrowed(&self.0)),
        })
    }
}
