use frender_common::utils::pin_project_map_array;

use crate::{render_state::array::ArrayRenderState, Element, RenderHtml};

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
