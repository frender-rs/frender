use std::pin::Pin;

use crate::{utils::pin_as_deref_mut, RenderState, UpdateRenderState};

pub struct BoxState<E, const DYN: bool>(pub E);

impl<Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for BoxState<E, false> {
    type State = Pin<Box<E::State>>;

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        E::update_render_state(self.0, ctx, pin_as_deref_mut(state))
    }
}

// impl<Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for BoxState<E, true> {
//     type State = Pin<Box<dyn RenderState>>;

//     fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
//         todo!()
//     }
// }