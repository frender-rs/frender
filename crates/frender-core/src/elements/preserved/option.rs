use std::pin::Pin;

use lazy_pinned::LazyPinned;

use crate::{RenderContext, RenderState, UpdateRenderState};

use super::Preserved;

pin_project_lite::pin_project! {
    pub struct PreservedOptionState<S> {
        #[pin]
        inner: LazyPinned<S>
    }
}

impl<Ctx: for<'ctx> RenderContext<'ctx>, S: RenderState<Ctx>> RenderState<Ctx>
    for PreservedOptionState<S>
{
    fn unmount(self: Pin<&mut Self>) {
        self.project().inner.as_pin_mut().map(S::unmount);
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project()
            .inner
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(()), |s| S::poll_reactive(s, ctx, cx))
    }
}

impl<Ctx: for<'ctx> RenderContext<'ctx>, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx>
    for Preserved<Option<R>>
{
    type State = PreservedOptionState<R::State>;

    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State {
        PreservedOptionState {
            inner: LazyPinned(self.0.map(|this| R::initialize_render_state(this, ctx))),
        }
    }

    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        state: Pin<&mut Self::State>,
    ) {
        if let Some(element) = self.0 {
            state.project().inner.use_pin_or_insert_with_data(
                (element, ctx),
                |(element, ctx), state| {
                    element.update_render_state(ctx, state);
                },
                |(element, ctx)| element.initialize_render_state(ctx),
            );
        } else {
            state.project().inner.as_pin_mut().map(R::State::unmount);
        }
    }
}
