use std::pin::Pin;

use crate::{RenderContext, RenderState, UpdateRenderState};

impl<Ctx: for<'ctx> RenderContext<'ctx>, S: RenderState<Ctx> + Unpin> RenderState<Ctx>
    for Option<S>
{
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
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.get_mut() {
            Some(s) => S::poll_reactive(Pin::new(s), ctx, cx),
            None => std::task::Poll::Ready(()),
        }
    }
}

impl<Ctx: for<'ctx> RenderContext<'ctx>, E: UpdateRenderState<Ctx>> UpdateRenderState<Ctx>
    for Option<E>
where
    <E as UpdateRenderState<Ctx>>::State: Unpin,
{
    type State = Option<E::State>;

    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State {
        match self {
            Some(this) => Some(this.initialize_render_state(ctx)),
            None => None,
        }
    }

    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        state: Pin<&mut Self::State>,
    ) {
        if let Some(this) = self {
            match state.get_mut() {
                Some(state) => this.update_render_state(ctx, Pin::new(state)),
                state => *state = Some(this.initialize_render_state(ctx)),
            };
        } else {
            <Option<E::State> as RenderState<Ctx>>::unmount(state)
        }
    }
}
