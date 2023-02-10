use std::pin::Pin;

use crate::{RenderState, UpdateRenderState};

pub enum Either<A, B> {
    A(A),
    B(B),
}

pin_project_lite::pin_project! {
    pub struct EitherState<A, B> {
        #[pin]
        pub a: Option<A>,
        #[pin]
        pub b: B,
    }
}

impl<A: RenderState, B: RenderState> RenderState for EitherState<A, B> {
    fn unmount(self: Pin<&mut Self>) {
        let this = self.project();
        match this.mounted {
            Some(Either::A(_)) => this.a.unmount(),
            Some(Either::B(_)) => this.b.unmount(),
            None => return,
        }
        *this.mounted = None;
    }

    fn poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let this = self.project();
        match this.mounted {
            Some(Either::A(_)) => this.a.poll_reactive(cx),
            Some(Either::B(_)) => this.b.poll_reactive(cx),
            None => true.into(),
        }
    }
}

impl<A, B, Ctx> UpdateRenderState<Ctx> for Either<A, B>
where
    A: UpdateRenderState<Ctx>,
    B: UpdateRenderState<Ctx>,
{
    type State = EitherState<A::State, B::State>;

    fn initialize_render_state(self, ctx: &mut Ctx) -> Self::State {
        match self {
            Either::A(a) => EitherState {
                mounted: (),
                a: a.initialize_render_state(ctx),
                b: None,
            },
            Either::B(b) => b.initialize_render_state(ctx),
        }
    }

    fn update_render_state(self, ctx: &mut Ctx, state: Pin<&mut Self::State>) {
        let state = state.project();
        match self {
            Either::A(this) => {
                if let Some(Either::B(_)) = state.mounted {
                    state.b.unmount();
                    *state.mounted = None;
                }
                this.update_render_state(ctx, state.a);
                *state.mounted = Some(Either::A(()));
            }
            Either::B(this) => {
                if let Some(Either::A(_)) = state.mounted {
                    state.a.unmount();
                    *state.mounted = None;
                }
                this.update_render_state(ctx, state.b);
                *state.mounted = Some(Either::B(()));
            }
        }
    }
}
