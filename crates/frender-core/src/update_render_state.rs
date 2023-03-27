use std::pin::Pin;

use crate::RenderState;

pub trait RenderContext<'ctx, ImplicitBounds = &'ctx Self> {
    type ContextData;
}

pub trait UpdateRenderState<Ctx: for<'ctx> RenderContext<'ctx>> {
    type State: RenderState<Ctx>;

    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State;

    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        state: Pin<&mut Self::State>,
    );
}
