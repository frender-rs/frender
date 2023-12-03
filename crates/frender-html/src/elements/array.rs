use std::pin::Pin;

use frender_common::utils::pin_project_map_array;

use crate::{Element, RenderHtml, RenderState};

impl<R, S: RenderState<R>, const N: usize> RenderState<R> for [S; N] {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {
        // pin_project_map_array_with_mut(self, S::unmount, renderer)
        pin_project_map_array(self, |s| s.unmount(renderer))
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        pin_project_map_array(self, S::state_unmount)
    }

    fn poll_render(self: std::pin::Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
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

pub struct ArrayRenderState<S, const N: usize>([S; N]);

impl<S, const N: usize> ArrayRenderState<S, N> {
    fn project_inner(self: Pin<&mut Self>) -> Pin<&mut [S; N]> {
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

    fn poll_render(self: Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        self.project_inner().poll_render(renderer, cx)
    }
}

impl<E: Element, const N: usize> Element for [E; N] {
    // type RenderState<R: RenderHtml> = [E::RenderState<R>; N];
    type RenderState<R: RenderHtml> = ArrayRenderState<E::RenderState<R>, N>;

    fn render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update(renderer, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_force_reposition(renderer, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_maybe_reposition(renderer, state, force_reposition));
        debug_assert!(this.next().is_none());
    }

    // type UnpinnedRenderState<R: RenderHtml> = [E::UnpinnedRenderState<R>; N];
    type UnpinnedRenderState<R: RenderHtml> = ArrayRenderState<E::UnpinnedRenderState<R>, N>;

    fn unpinned_render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update(renderer, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_force_reposition(renderer, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>, force_reposition: bool) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_maybe_reposition(renderer, &mut render_state[i], force_reposition);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }
}
