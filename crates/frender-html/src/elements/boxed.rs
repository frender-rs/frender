use std::pin::Pin;

use crate::{Element, RenderHtml};

impl<E: Element> Element for Box<E> {
    type RenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E::RenderState<PEH, R>;

    fn render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>) {
        E::render_update(*self, peh, renderer, render_state)
    }

    fn render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>) {
        E::render_update_force_reposition(*self, peh, renderer, render_state)
    }

    fn render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<PEH, Renderer>>, force_reposition: bool) {
        E::render_update_maybe_reposition(*self, peh, renderer, render_state, force_reposition)
    }

    type UnpinnedRenderState<PEH: ?Sized, R: RenderHtml + ?Sized> = E::UnpinnedRenderState<PEH, R>;

    fn unpinned_render_update<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        E::unpinned_render_update(*self, peh, renderer, render_state)
    }

    fn unpinned_render_update_force_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(self, peh: &mut PEH, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>) {
        E::unpinned_render_update_force_reposition(*self, peh, renderer, render_state)
    }

    fn unpinned_render_update_maybe_reposition<PEH: ?Sized, Renderer: RenderHtml + ?Sized>(
        self,
        peh: &mut PEH,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<PEH, Renderer>,
        force_reposition: bool,
    ) {
        E::unpinned_render_update_maybe_reposition(*self, peh, renderer, render_state, force_reposition)
    }
}
