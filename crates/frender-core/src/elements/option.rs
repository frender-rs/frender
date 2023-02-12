use std::pin::Pin;

use crate::{RenderState, UpdateRenderState};

impl<S: RenderState + Unpin> RenderState for Option<S> {
    fn unmount(self: Pin<&mut Self>) {
        let this = self.get_mut();
        match this {
            Some(state) => {
                S::unmount(Pin::new(state));
            }
            None => return,
        }
        *this = None;
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        match self.get_mut() {
            Some(s) => S::poll_reactive(Pin::new(s), cx),
            None => std::task::Poll::Ready(false),
        }
    }
}

impl<Ctx, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx> for Option<E>
where
    <E as UpdateRenderState<Ctx>>::State: Unpin,
{
    type State = Option<E::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        match self {
            Some(this) => Some(this.initialize_render_state(ctx)),
            None => todo!(),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        match (self, state.get_mut()) {
            (Some(this), Some(state)) => this.update_render_state(ctx, Pin::new(state)),
            (Some(this), state @ None) => *state = Some(this.initialize_render_state(ctx)),
            (None, Some(state)) => Pin::new(state).unmount(),
            (None, None) => {}
        }
    }
}
