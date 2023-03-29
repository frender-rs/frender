use std::pin::Pin;

use crate::UpdateRenderState;

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Box<R> {
    type State = R::State;

    #[inline]
    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        R::initialize_render_state(*self, ctx)
    }

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        R::update_render_state(*self, ctx, state)
    }
}
