use frender_common::utils::pin_project_map_array;

use crate::{render_state::array::ArrayRenderState, Element, RenderHtml};

impl<E: Element, const N: usize> Element for [E; N] {
    // type RenderState<R: RenderHtml> = [E::RenderState<R>; N];
    type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = ArrayRenderState<E::RenderState<PEH, R>, N>;

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update(peh, renderer, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_force_reposition(peh, renderer, state));
        debug_assert!(this.next().is_none());
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,

        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: std::pin::Pin<&mut Self::RenderState<PEH, Renderer>>,
        force_reposition: bool,
    ) {
        let render_state = render_state.project_inner();
        let mut this = self.into_iter();
        pin_project_map_array(render_state, |state| this.next().unwrap().render_update_maybe_reposition(peh, renderer, state, force_reposition));
        debug_assert!(this.next().is_none());
    }

    // type UnpinnedRenderState<R: RenderHtml> = [E::UnpinnedRenderState<R>; N];
    type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = ArrayRenderState<E::UnpinnedRenderState<PEH, R>, N>;

    fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update(peh, renderer, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_force_reposition(peh, renderer, &mut render_state[i]);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }

    fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,

        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        force_reposition: bool,
    ) {
        let render_state = &mut render_state.0;

        let mut this = self.into_iter();
        let mut i = 0;

        while i < N {
            this.next().unwrap().unpinned_render_update_maybe_reposition(peh, renderer, &mut render_state[i], force_reposition);
            i += 1;
        }

        debug_assert!(this.next().is_none());
    }
}
