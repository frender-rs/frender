use std::pin::Pin;
use std::task::Poll;

use frender_dom::behaviors::ElementWithChildren;
use frender_dom::ui_handle::UiHandle;
use frender_dom::{RenderStateWithAnyParent, RenderStateWithParentElementsHandle, StateUnmount};

use crate::element::{PinMutRenderInitStatesOfKind, PinnedMutRenderStatesOfKind, PinnedRenderStateKind, PinnedUiHandleOfKind, UnpinnedMutRenderStatesOfKind, UnpinnedRenderStateKind, UnpinnedRenderStatesOfKind};
use crate::{BehaviorType, RenderHtml};

use crate::html::behavior_type_traits;
use crate::Element;

pub trait CsrComponentNormalElement: behavior_type_traits::Element {}

pub trait RenderStateKindPollRenderWithParent<ParentType: ?Sized + BehaviorType>: UnpinnedRenderStateKind + PinnedRenderStateKind {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        states: PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        states: UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait CsrComponent<Children>: behavior_type_traits::Element {
    type ChildrenRenderStateKind: RenderStateKindPollRenderWithParent<Self>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::Element<R>,
        children_states: PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>;

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::Element<R>,
        children_states: PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    );

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::Element<R>,
    ) -> UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R>;

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::Element<R>,
        children_states: UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    );
}

#[cfg(todo)]
enum Never {}

#[cfg(todo)]
pub struct KindRenderStateWithAnyParent<K: crate::RenderStateKind>(Never, std::marker::PhantomData<K>);

#[cfg(todo)]
impl<K: crate::RenderStateKind, ElType: ?Sized + behavior_type_traits::Element> RenderStateWithPehKind<ElType> for KindRenderStateWithAnyParent<K> {
    type RenderStateWithPeh<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<<K as crate::RenderStateKind>::RenderStatePinned<R>>;
    type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = RenderStateWithAnyParent<<K as crate::RenderStateKind>::RenderStateUnpinned<R>>;
}

#[cfg(todo)]
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
