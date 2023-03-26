use crate::{utils::pin_project_map_array, RenderContext, RenderState, UpdateRenderState};

impl<Ctx: for<'ctx> RenderContext<'ctx>, S: RenderState<Ctx>, const N: usize> RenderState<Ctx>
    for [S; N]
{
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        pin_project_map_array(self, S::unmount)
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        pin_project_map_array(self, |state| match S::poll_reactive(state, ctx, cx) {
            std::task::Poll::Ready(()) => {}
            v @ std::task::Poll::Pending => {
                if let std::task::Poll::Ready(()) = res {
                    res = v;
                }
            }
        });

        res
    }
}

impl<E: UpdateRenderState<Ctx>, Ctx: for<'ctx> RenderContext<'ctx>, const N: usize>
    UpdateRenderState<Ctx> for [E; N]
{
    type State = [E::State; N];

    fn initialize_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
    ) -> Self::State {
        self.map(|v| E::initialize_render_state(v, ctx))
    }

    fn update_render_state(
        self,
        ctx: &mut <Ctx as RenderContext<'_>>::ContextData,
        state: std::pin::Pin<&mut Self::State>,
    ) {
        let mut this = self.into_iter();
        pin_project_map_array(state, |state| {
            this.next().unwrap().update_render_state(ctx, state)
        });
        debug_assert!(this.next().is_none());
    }
}
