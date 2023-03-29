use std::pin::Pin;

use either::Either;

use crate::{RenderState, UpdateRenderState};

impl<L: RenderState, R: RenderState> RenderState for Either<L, R> {
    fn unmount(self: Pin<&mut Self>) {
        match self.as_pin_mut() {
            Either::Left(s) => s.unmount(),
            Either::Right(s) => s.unmount(),
        }
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_reactive(cx),
            Either::Right(s) => s.poll_reactive(cx),
        }
    }
}

impl<L, R, Ctx> UpdateRenderState<Ctx> for Either<L, R>
where
    L: UpdateRenderState<Ctx>,
    R: UpdateRenderState<Ctx>,
    L::State: Unpin,
    R::State: Unpin,
{
    type State = Either<L::State, R::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        match self {
            Either::Left(e) => Either::Left(e.initialize_render_state(ctx)),
            Either::Right(e) => Either::Right(e.initialize_render_state(ctx)),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, mut state: Pin<&mut Self::State>) {
        *state = match (self, state.as_mut().as_pin_mut()) {
            (Either::Left(e), Either::Left(state)) => return e.update_render_state(ctx, state),
            (Either::Right(e), Either::Right(state)) => return e.update_render_state(ctx, state),
            (Either::Left(e), Either::Right(state)) => {
                state.unmount();
                Either::Left(e.initialize_render_state(ctx))
            }
            (Either::Right(e), Either::Left(state)) => {
                state.unmount();
                Either::Right(e.initialize_render_state(ctx))
            }
        };
    }
}
