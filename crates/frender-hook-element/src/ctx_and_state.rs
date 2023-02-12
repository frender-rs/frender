use std::pin::Pin;

use frender_core::{RenderState, UpdateRenderState};
use lazy_pinned::LazyPinned;

pub struct ContextAndState<'a, Ctx, State> {
    context: &'a mut Ctx,
    state: Pin<&'a mut LazyPinned<State>>,
}

impl<'a, Ctx, State> ContextAndState<'a, Ctx, State> {
    pub fn new(context: &'a mut Ctx, state: Pin<&'a mut LazyPinned<State>>) -> Self {
        Self { context, state }
    }
}

impl<'a, Ctx, S: RenderState> ContextAndState<'a, Ctx, S> {
    pub fn render<E: UpdateRenderState<Ctx, State = S>>(self, element: E) {
        self.state.use_pin_or_insert_with_data(
            (element, self.context),
            |(element, context), state| element.update_render_state(context, state),
            |(element, context)| element.initialize_render_state(context),
        );
    }
}
