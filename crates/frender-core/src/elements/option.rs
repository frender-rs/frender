use std::pin::Pin;

use crate::{utils::LazyPinned, RenderState, UpdateRenderState};

pin_project_lite::pin_project! {
    pub struct OptionState<S> {
        #[pin]
        inner: LazyPinned<S>
    }
}

impl<S: RenderState> RenderState for OptionState<S> {
    fn unmount(self: Pin<&mut Self>) {
        self.project().inner.as_pin_mut().map(S::unmount);
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        self.project()
            .inner
            .as_pin_mut()
            .map_or(std::task::Poll::Ready(false), |s| S::poll_reactive(s, cx))
    }
}

impl<Ctx, R: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Option<R> {
    type State = OptionState<R::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        OptionState {
            inner: LazyPinned(self.map(|this| R::initialize_render_state(this, ctx))),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        if let Some(element) = self {
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
