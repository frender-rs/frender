use frender_html::RenderHtml;

use crate::Element;

pub trait CsrComponentNormalElement {}

pub trait CsrComponent<Children> {
    type ChildrenRenderState<R: RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<R: RenderHtml>(
        children: Children,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
    );

    type ChildrenUnpinnedRenderState<R: RenderHtml>: crate::RenderState<R> + Default + Unpin;

    fn children_unpinned_render_update<R: RenderHtml>(
        children: Children,
        renderer: &mut R,
        children_state: &mut Self::ChildrenUnpinnedRenderState<R>,
    );
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderState<R: RenderHtml> = Children::RenderState<R>;

    fn children_render_update<R: RenderHtml>(
        children: Children,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
    ) {
        Children::render_update(children, renderer, children_state)
    }

    type ChildrenUnpinnedRenderState<R: RenderHtml> = Children::UnpinnedRenderState<R>;

    fn children_unpinned_render_update<R: RenderHtml>(
        children: Children,
        renderer: &mut R,
        children_state: &mut Self::ChildrenUnpinnedRenderState<R>,
    ) {
        Children::unpinned_render_update(children, renderer, children_state)
    }
}

// TODO: remove
impl<C> CsrComponentNormalElement for C {}
