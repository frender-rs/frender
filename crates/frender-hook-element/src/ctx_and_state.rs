use std::pin::Pin;

use frender_core::{RenderState, UpdateRenderState};
use lazy_pinned::LazyPinned;

use crate::HookContext;

pin_project_lite::pin_project! {
    pub(crate) struct ContextAndStateData<Ctx, State> {
        context: Ctx,
        #[pin]
        state: LazyPinned<State>,
    }
}

impl<Ctx, State> ContextAndStateData<Ctx, State> {
    pub(crate) fn update_context(mut self: Pin<&mut Self>, context: Ctx) -> Pin<&mut Self> {
        *self.as_mut().project().context = context;
        self
    }

    pub(crate) fn as_ctx_and_state(self: Pin<&mut Self>) -> ContextAndState<'_, Ctx, State> {
        let this = self.project();
        ContextAndState {
            context: this.context,
            state: this.state,
        }
    }

    pub(crate) fn with_context_and_state(
        self: Pin<&mut Self>,
        f: impl FnOnce(ContextAndState<'_, Ctx, State>),
    ) where
        Ctx: HookContext,
    {
        let this = self.project();

        Ctx::with_context(this.context, |context| {
            f(ContextAndState::new(context, this.state))
        })
    }
}

impl<Ctx, State> RenderState for ContextAndStateData<Ctx, State>
where
    State: RenderState,
{
    fn unmount(self: Pin<&mut Self>) {
        if let Some(state) = self.project().state.as_pin_mut() {
            state.unmount()
        }
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        self.project()
            .state
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(true), |s| {
                State::poll_reactive(s, cx)
            })
    }
}

impl<Ctx, State> ContextAndStateData<Ctx, State> {
    pub fn new(ctx: Ctx) -> Self {
        Self {
            context: ctx,
            state: LazyPinned(None),
        }
    }
}

pub struct Rendered<S>(std::marker::PhantomData<S>);

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

impl<'a, Ctx, S: RenderState> ContextAndState<'a, Ctx, S> {
    pub fn render<E: UpdateRenderState<Ctx, State = S>>(self, element: E) -> Rendered<S> {
        self.state.use_pin_or_insert_with_data(
            (element, self.context),
            |(element, context), state| element.update_render_state(context, state),
            |(element, context)| element.initialize_render_state(context),
        );

        Rendered(std::marker::PhantomData)
    }
}
