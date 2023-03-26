use std::pin::Pin;

use crate::{RenderContext, UpdateRenderState};

impl<Ctx: for<'ctx> RenderContext<'ctx>, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx>
    for Box<R>
{
    type State = R::State;

    #[inline]
    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State {
        R::initialize_render_state(*self, ctx)
    }

    #[inline]
    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        state: Pin<&mut Self::State>,
    ) {
        R::update_render_state(*self, ctx, state)
    }
}
