use frender_dom::behaviors::ElementWithChildren;
use frender_dom::{RenderStateWithAnyParent, RenderStateWithParentElementsHandle};

use crate::RenderHtml;

use crate::html::behavior_type_traits;
use crate::Element;

pub trait CsrComponentNormalElement: behavior_type_traits::Element {}

pub trait RenderStateWithPehKind<ElType: ?Sized + behavior_type_traits::Element> {
    type RenderStateWithPeh<R: RenderHtml + ?Sized>: RenderStateWithParentElementsHandle<ElType::Element<R>, R> + Default;
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized>: RenderStateWithParentElementsHandle<ElType::Element<R>, R> + Default + Unpin;
}

pub trait CsrComponent<Children>: behavior_type_traits::Element {
    type ChildrenRenderStateKind: RenderStateWithPehKind<Self>;

    fn children_render_update<R: RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
    );

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
    );
}

enum Never {}
pub struct KindRenderStateWithAnyParent<K: crate::RenderStateKind>(Never, std::marker::PhantomData<K>);

impl<K: crate::RenderStateKind, ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindRenderStateWithAnyParent<K> {
    type RenderStateWithPeh<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<<K as crate::RenderStateKindPinned>::RenderState<R>>;
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<<K as crate::RenderStateKindUnpinned>::UnpinnedRenderState<R>>;
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderStateKind = KindRenderStateWithAnyParent<Children::RenderStateKind>;

    fn children_render_update<R: RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
    ) {
        element.with_render_context_at_first_child_of_self(renderer, |renderer| Children::render_update(children, renderer, children_state.as_pin_mut()))
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        self,
        children: Children,
        element: &mut Self::Element<R>,
        renderer: &mut R,
        children_state: &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
    ) {
        element.with_render_context_at_first_child_of_self(renderer, |renderer| Children::unpinned_render_update(children, renderer, &mut children_state.render_state))
    }
}
