use std::pin::Pin;

use crate::{
    utils::{pin_as_deref_mut, pin_downcast_mut},
    UpdateRenderState,
};

pub struct BoxState<E, const DYN: bool = true>(pub E);

impl<Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for BoxState<E, false> {
    type State = Pin<Box<E::State>>;

    #[inline]
    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        Box::pin(self.0.initialize_render_state(ctx))
    }

    #[inline]
    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        E::update_render_state(self.0, ctx, pin_as_deref_mut(state))
    }
}

impl<Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for BoxState<E, true>
where
    Ctx: 'static, // TODO: why this is required
    E::State: std::any::Any,
{
    type State = Pin<Box<dyn crate::AnyRenderState<Ctx>>>;

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        let state = pin_downcast_mut(pin_as_deref_mut(state).pin_as_mut_any())
            .expect("state type should match");
        E::update_render_state(self.0, ctx, state)
    }

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        Box::pin(E::initialize_render_state(self.0, ctx))
    }
}
