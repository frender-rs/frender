use std::pin::Pin;

use frender_csr::{RenderState, UpdateRenderState};
use lazy_pinned::LazyPinned;

pub(crate) fn lazy_pinned_state_unmount<State: RenderState<Ctx>, Ctx>(
    state: Pin<&mut LazyPinned<State>>,
) {
    if let Some(state) = state.as_pin_mut() {
        state.unmount()
    }
}

pub(crate) fn lazy_pinned_state_poll_reactive<State: RenderState<Ctx>, Ctx>(
    state: Pin<&mut LazyPinned<State>>,
    ctx: &mut Ctx,
    cx: &mut std::task::Context<'_>,
) -> std::task::Poll<()> {
    state.as_pin_mut().map_or(std::task::Poll::Ready(()), |s| {
        State::poll_reactive(s, ctx, cx)
    })
}

pub struct Rendered<S>(std::marker::PhantomData<S>);

impl<S> Rendered<S> {
    pub(crate) fn _new() -> Self {
        Self(std::marker::PhantomData)
    }
}

pub struct ContextAndState<'a, Ctx, State> {
    context: &'a mut Ctx,
    state: Pin<&'a mut LazyPinned<State>>,
}

impl<'a, Ctx, State> ContextAndState<'a, Ctx, State> {
    pub fn context(&self) -> &Ctx {
        self.context
    }
}

impl<'a, Ctx, State> ContextAndState<'a, Ctx, State> {
    pub fn new(context: &'a mut Ctx, state: Pin<&'a mut LazyPinned<State>>) -> Self {
        Self { context, state }
    }
}

impl<'a, Ctx, S: RenderState<Ctx>> ContextAndState<'a, Ctx, S> {
    pub fn render<E: UpdateRenderState<Ctx, State = S>>(self, element: E) -> Rendered<S> {
        self.state.use_pin_or_insert_with_data(
            (element, self.context),
            |(element, context), state| element.update_render_state(context, state),
            |(element, context)| element.initialize_render_state(context),
        );

        Rendered(std::marker::PhantomData)
    }
}
