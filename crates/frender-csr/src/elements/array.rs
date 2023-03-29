use crate::{utils::pin_project_map_array, Element, RenderState};

impl<S: RenderState, const N: usize> RenderState for [S; N] {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>) {
        pin_project_map_array(self, S::unmount)
    }

    fn poll_csr(
        self: std::pin::Pin<&mut Self>,
        ctx: &mut crate::CsrContext,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        pin_project_map_array(self, |state| match S::poll_csr(state, ctx, cx) {
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

impl<E: Element, const N: usize> Element for [E; N] {
    type CsrState = [E::CsrState; N];

    fn into_csr_state(self, ctx: &mut crate::CsrContext) -> Self::CsrState {
        self.map(|v| E::into_csr_state(v, ctx))
    }

    fn update_csr_state(self, ctx: &mut crate::CsrContext, state: std::pin::Pin<&mut Self::CsrState>) {
        let mut this = self.into_iter();
        pin_project_map_array(state, |state| {
            this.next().unwrap().update_csr_state(ctx, state)
        });
        debug_assert!(this.next().is_none());
    }
}
