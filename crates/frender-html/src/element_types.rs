use crate::RenderHtml;

use crate::html::behavior_type_traits;
use crate::Element;

pub trait CsrComponentNormalElement: behavior_type_traits::Element {}

pub trait CsrComponent<Children>: behavior_type_traits::Element {
    type ChildrenRenderState<R: RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<R: RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>);

    type ChildrenUnpinnedRenderState<R: RenderHtml>: crate::RenderState<R> + Default + Unpin;

    fn children_unpinned_render_update<R: RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>);
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderState<R: RenderHtml> = Children::RenderState<R>;

    fn children_render_update<R: RenderHtml>(children: Children, _: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
        Children::render_update(children, renderer, children_state)
    }

    type ChildrenUnpinnedRenderState<R: RenderHtml> = Children::UnpinnedRenderState<R>;

    fn children_unpinned_render_update<R: RenderHtml>(children: Children, _: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
        Children::unpinned_render_update(children, renderer, children_state)
    }
}
