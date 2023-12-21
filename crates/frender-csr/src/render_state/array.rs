use std::pin::Pin;

use frender_common::utils::pin_project_map_array;

use crate::RenderState;

impl<R, S: RenderState<R>, const N: usize> RenderState<R> for [S; N] {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        // pin_project_map_array_with_mut(self, S::unmount, renderer)
        pin_project_map_array(self, |s| s.unmount(renderer))
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        pin_project_map_array(self, S::state_unmount)
    }

    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let mut res = std::task::Poll::Ready(());

        pin_project_map_array(self, |state| match S::poll_render(state, renderer, cx) {
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

pub struct ArrayRenderState<S, const N: usize>(pub [S; N]);

impl<S, const N: usize> ArrayRenderState<S, N> {
    pub fn project_inner(self: Pin<&mut Self>) -> Pin<&mut [S; N]> {
        // SAFETY: pin_projection
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) }
    }
}

impl<S: Default, const N: usize> Default for ArrayRenderState<S, N> {
    fn default() -> Self {
        Self([(); N].map(|()| Default::default()))
    }
}

impl<R, S: RenderState<R>, const N: usize> RenderState<R> for ArrayRenderState<S, N> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        self.project_inner().unmount(renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project_inner().state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project_inner().poll_render(renderer, cx)
    }
}
