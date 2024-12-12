use std::pin::Pin;

use frender_common::utils::{
    pin_project_for_each_array, pin_project_iter_mut_array, pin_project_map_array,
};
use pin_project_lite::pin_project;

use crate::{RenderState, StateUnmount};

impl<R: ?Sized, S: RenderState<R>, const N: usize> RenderState<R> for [S; N] {
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

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        self.iter()
            .for_each(|state| state.check_and_move_cursor(render_context))
    }
}

pub struct ArrayRenderState<S, const N: usize>(pub [S; N]);

impl<S, const N: usize> ArrayRenderState<S, N> {
    pub fn project_inner(self: Pin<&mut Self>) -> Pin<&mut [S; N]> {
        // SAFETY: pin_projection
        unsafe { Pin::new_unchecked(&mut self.get_unchecked_mut().0) }
    }

    pub fn iter_pin_mut(
        self: Pin<&mut Self>,
    ) -> impl Iterator<Item = Pin<&mut S>> + ExactSizeIterator {
        pin_project_iter_mut_array(self.project_inner())
    }
}

/// This is not always optimized as documented by `<[_; N]>::map()`.
impl<S: Default, const N: usize> Default for ArrayRenderState<S, N> {
    fn default() -> Self {
        Self(::core::array::from_fn(|_| Default::default()))
    }
}

impl<S: StateUnmount, const N: usize> StateUnmount for ArrayRenderState<S, N> {
    fn state_unmount(self: Pin<&mut Self>) {
        pin_project_for_each_array(self.project_inner(), S::state_unmount)
    }
}
