use std::pin::Pin;

use either::Either;

use crate::{Element, RenderState};

impl<L: RenderState, R: RenderState> RenderState for Either<L, R> {
    fn unmount(self: Pin<&mut Self>) {
        match self.as_pin_mut() {
            Either::Left(s) => s.unmount(),
            Either::Right(s) => s.unmount(),
        }
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_csr(ctx, cx),
            Either::Right(s) => s.poll_csr(ctx, cx),
        }
    }
}

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type CsrState = Either<L::CsrState, R::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        match self {
            Either::Left(e) => Either::Left(e.into_csr_state(ctx)),
            Either::Right(e) => Either::Right(e.into_csr_state(ctx)),
        }
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, mut state: Pin<&mut Self::CsrState>) {
        let new_state = match (self, state.as_mut().as_pin_mut()) {
            (Either::Left(e), Either::Left(state)) => return e.update_csr_state(ctx, state),
            (Either::Right(e), Either::Right(state)) => return e.update_csr_state(ctx, state),
            (Either::Left(e), Either::Right(state)) => {
                state.unmount();
                Either::Left(e.into_csr_state(ctx))
            }
            (Either::Right(e), Either::Left(state)) => {
                state.unmount();
                Either::Right(e.into_csr_state(ctx))
            }
        };

        state.set(new_state);
    }
}