use frender_dom::RenderStateWithAnyParent;

use crate::RenderHtml;

use crate::html::behavior_type_traits;
use crate::Element;

pub trait CsrComponentNormalElement: behavior_type_traits::Element {}

pub trait CsrComponent<Children>: behavior_type_traits::Element {
    type ChildrenRenderState<R: RenderHtml + ?Sized>: crate::RenderStateWithParentElementsHandle<Self::Element<R>, R> + Default;

    fn children_render_update<R: RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>);

    type ChildrenUnpinnedRenderState<R: RenderHtml + ?Sized>: crate::RenderStateWithParentElementsHandle<Self::Element<R>, R> + Default + Unpin;

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>);
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderState<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<Children::RenderState<R>>;

    fn children_render_update<R: RenderHtml + ?Sized>(children: Children, _: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
        Children::render_update(children, renderer, children_state.as_pin_mut())
    }

    type ChildrenUnpinnedRenderState<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<Children::UnpinnedRenderState<R>>;

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(children: Children, _: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
        Children::unpinned_render_update(children, renderer, &mut children_state.render_state)
    }
}
