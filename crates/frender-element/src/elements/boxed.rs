use std::pin::Pin;

use crate::Element;

impl<E: Element> Element for Box<E> {
    type RenderState<R: frender_html::RenderHtml> = E::RenderState<R>;

    fn render_update<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) {
        E::render_update(*self, renderer, render_state)
    }

    fn render_update_force_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
    ) {
        E::render_update_force_reposition(*self, renderer, render_state)
    }

    fn render_update_maybe_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: Pin<&mut Self::RenderState<Renderer>>,
        force_reposition: bool,
    ) {
        E::render_update_maybe_reposition(*self, renderer, render_state, force_reposition)
    }

    type UnpinnedRenderState<R: frender_html::RenderHtml> = E::UnpinnedRenderState<R>;

    fn unpinned_render_update<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) {
        E::unpinned_render_update(*self, renderer, render_state)
    }

    fn unpinned_render_update_force_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
    ) {
        E::unpinned_render_update_force_reposition(*self, renderer, render_state)
    }

    fn unpinned_render_update_maybe_reposition<Renderer: frender_html::RenderHtml>(
        self,
        renderer: &mut Renderer,
        render_state: &mut Self::UnpinnedRenderState<Renderer>,
        force_reposition: bool,
    ) {
        E::unpinned_render_update_maybe_reposition(*self, renderer, render_state, force_reposition)
    }
}
