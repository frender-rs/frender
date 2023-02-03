use crate::{utils::pin_project_map_array, RenderState, UpdateRenderState};

impl<S: RenderState, const N: usize> RenderState for [S; N] {
    #[inline]
    fn new_uninitialized() -> Self {
        [(); N].map(|_| S::new_uninitialized())
    }

    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        pin_project_map_array(self, S::unmount)
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool> {
        let mut res = std::task::Poll::Ready(false);

        pin_project_map_array(self, |state| match S::poll_reactive(state, cx) {
            std::task::Poll::Ready(false) => {}
            v @ std::task::Poll::Ready(true) => res = v,
            v @ std::task::Poll::Pending => {
                if let std::task::Poll::Ready(false) = res {
                    res = v;
                }
            }
        });

        res
    }
}

impl<E: UpdateRenderState<Ctx>, Ctx, const N: usize> UpdateRenderState<Ctx> for [E; N] {
    type State = [E::State; N];

    fn update_render_state(self, ctx: &mut Ctx, state: std::pin::Pin<&mut Self::State>) {
        let mut this = self.into_iter();
        pin_project_map_array(state, |state| {
            this.next().unwrap().update_render_state(ctx, state)
        });
        debug_assert!(this.next().is_none());
    }
}
