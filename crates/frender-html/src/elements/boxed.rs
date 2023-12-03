use std::pin::Pin;

use crate::{Element, RenderHtml};

impl<E: Element> Element for Box<E> {
    type RenderState<R: RenderHtml> = E::RenderState<R>;

    fn render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        E::render_update(*self, renderer, render_state)
    }

    fn render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>) {
        E::render_update_force_reposition(*self, renderer, render_state)
    }

    fn render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: Pin<&mut Self::RenderState<Renderer>>, force_reposition: bool) {
        E::render_update_maybe_reposition(*self, renderer, render_state, force_reposition)
    }

    type UnpinnedRenderState<R: RenderHtml> = E::UnpinnedRenderState<R>;

    fn unpinned_render_update<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>) {
        E::unpinned_render_update(*self, renderer, render_state)
    }

    fn unpinned_render_update_force_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>) {
        E::unpinned_render_update_force_reposition(*self, renderer, render_state)
    }

    fn unpinned_render_update_maybe_reposition<Renderer: RenderHtml>(self, renderer: &mut Renderer, render_state: &mut Self::UnpinnedRenderState<Renderer>, force_reposition: bool) {
        E::unpinned_render_update_maybe_reposition(*self, renderer, render_state, force_reposition)
    }
}
