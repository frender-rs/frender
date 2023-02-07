use std::{any::Any, pin::Pin};

use frender_core::{RenderState, UpdateRenderState};

pub struct ContextAndState<'a, Ctx, State: ?Sized> {
    context: &'a mut Ctx,
    state: Pin<&'a mut State>,
}

impl<'a, Ctx, State: ?Sized> ContextAndState<'a, Ctx, State> {
    pub fn new(context: &'a mut Ctx, state: Pin<&'a mut State>) -> Self {
        Self { context, state }
    }
}

impl<'a, Ctx, S: RenderState + 'static> ContextAndState<'a, Ctx, S> {
    #[inline]
    pub fn render<E: UpdateRenderState<Ctx, State = S>>(
        mut self,
        element: E,
    ) -> ContextAndState<'a, Ctx, E::State> {
        // let mut this = self.downcast_state::<E::State>().unwrap();
        element.update_render_state(self.context, self.state.as_mut());

        self
    }
}
