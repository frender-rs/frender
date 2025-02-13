use std::pin::Pin;

use frender_common::reactive_value::RenderInitPinned;
use frender_dom::{behaviors::NodeRenderSelf, render::Render};

use crate::HtmlRenderContext;

pub struct RenderInit<T>(pub(crate) T);

pub trait RenderCursorPlaceholder {
    type CursorPlaceholder;
    fn render_cursor_placeholder(&mut self) -> Self::CursorPlaceholder;
}

impl<Ctx: ?Sized + HtmlRenderContext> RenderCursorPlaceholder for &mut Ctx {
    type CursorPlaceholder = <Ctx::Renderer as Render>::CursorPlaceholder;
    fn render_cursor_placeholder(&mut self) -> Self::CursorPlaceholder {
        self.map_mut_render_context(|render_context| NodeRenderSelf::render_self(render_context))
    }
}

impl<
        //
        T: RenderInitPinned<R, S>,
        R: RenderCursorPlaceholder,
        S: ?Sized,
    > RenderInitPinned<R, S> for RenderInit<T>
{
    type Output = (R::CursorPlaceholder, T::Output);

    fn render_init_pinned(self, mut renderer: R, state: Pin<&mut S>) -> Self::Output {
        (
            //
            renderer.render_cursor_placeholder(),
            self.0.render_init_pinned(renderer, state),
        )
    }
}
