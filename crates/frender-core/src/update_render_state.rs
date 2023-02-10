use std::pin::Pin;

use crate::RenderState;

pub trait UpdateRenderState<Ctx> {
    type State: RenderState;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>);
}
