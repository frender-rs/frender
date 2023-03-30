use std::pin::Pin;

use either::Either;
use lazy_pinned::LazyPinned;

use crate::{Element, RenderState};

pin_project_lite::pin_project! {
    pub struct PreservedEitherState<L, R> {
        left_is_mounted: Option<bool>,
        #[pin]
        left: LazyPinned<L>,
        #[pin]
        right: LazyPinned<R>,
    }
}

impl<L: RenderState, R: RenderState> RenderState for PreservedEitherState<L, R> {
    fn unmount(self: Pin<&mut Self>) {
        let this = self.project();
        match this.left_is_mounted {
            Some(true) => this.left.as_pin_mut().map(L::unmount),
            Some(false) => this.right.as_pin_mut().map(R::unmount),
            None => return,
        };
        *this.left_is_mounted = None;
    }

    fn poll_csr(
        self: Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let this = self.project();
        match this.left_is_mounted {
            Some(true) => this.left.as_pin_mut().unwrap().poll_csr(ctx, cx),
            Some(false) => this.right.as_pin_mut().unwrap().poll_csr(ctx, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}

impl<L, R> Element for super::Preserved<Either<L, R>>
where
    L: Element,
    R: Element,
{
    type CsrState = PreservedEitherState<L::CsrState, R::CsrState>;

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        match self.0 {
            Either::Left(e) => PreservedEitherState {
                left_is_mounted: Some(true),
                left: LazyPinned(Some(e.into_csr_state(ctx))),
                right: LazyPinned(None),
            },
            Either::Right(e) => PreservedEitherState {
                left_is_mounted: Some(true),
                left: LazyPinned(None),
                right: LazyPinned(Some(e.into_csr_state(ctx))),
            },
        }
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: Pin<&mut Self::CsrState>) {
        let state = state.project();
        match self.0 {
            Either::Left(e) => {
                if let Some(false) = state.left_is_mounted {
                    state.right.as_pin_mut().unwrap().unmount();
                }
                state.left.use_pin_or_insert_with_data(
                    (e, ctx),
                    |(e, ctx), state| e.update_csr_state(ctx, state),
                    |(e, ctx)| e.into_csr_state(ctx),
                );
                *state.left_is_mounted = Some(true);
            }
            Either::Right(e) => {
                if let Some(true) = state.left_is_mounted {
                    state.left.as_pin_mut().unwrap().unmount();
                }
                state.right.use_pin_or_insert_with_data(
                    (e, ctx),
                    |(e, ctx), state| e.update_csr_state(ctx, state),
                    |(e, ctx)| e.into_csr_state(ctx),
                );
                *state.left_is_mounted = Some(false);
            }
        }
    }
}
