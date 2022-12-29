use std::pin::Pin;

use crate::{RenderState, UpdateRenderState};

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Option<R> {
    type State = R::State;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        if let Some(element) = self {
            element.update_render_state(ctx, state);
        } else {
            R::State::unmount(state);
        }
    }
}
