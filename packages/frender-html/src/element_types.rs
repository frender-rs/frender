use std::pin::Pin;
use std::task::Poll;

use frender_common::convert::FromMut as _;
use frender_dom::behaviors::ElementWithChildren;

use crate::element::{
    PinnedRenderInitKind, PinnedRenderStateKind, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, RenderStateKind, UnpinnedRenderStateKind, UnpinnedStateOfKind, UnpinnedUiHandleOfKind,
    UnpinnedUnmountedUiHandleOfKind,
};
use crate::{BehaviorType, RenderHtml};

use crate::html::behavior_type_traits;
use crate::Element;

pub trait CsrComponentNormalElement: behavior_type_traits::Element {}

pub trait RenderStateKindPollRenderWithParent<ParentType: ?Sized + BehaviorType>: UnpinnedRenderStateKind + PinnedRenderStateKind {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        state: Pin<&mut PinnedStateOfKind<R, Self>>,
        ui_handle: &mut PinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        state: &mut UnpinnedStateOfKind<R, Self>,
        ui_handle: &mut UnpinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}

pub trait RenderInitWithParent<ParentType: ?Sized + BehaviorType, R: ?Sized + RenderHtml> {
    type PinnedRenderStateKind: PinnedRenderStateKind;
    fn render_init_pinned_with_parent(
        //
        self,
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        children_state: Pin<&mut PinnedStateOfKind<R, Self::PinnedRenderStateKind>>,
    );
}

pub trait PinnedRenderInitKindWithParent<ParentType: ?Sized + BehaviorType> {
    type PinnedRenderInitWithParent<R: ?Sized + RenderHtml>: RenderInitWithParent<ParentType, R>;
}

pub trait CsrComponent<Children>: BehaviorType {
    type ChildrenRenderStateKind: RenderStateKindPollRenderWithParent<Self> + PinnedRenderInitKindWithParent<Self>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
    ) -> (
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        <Self::ChildrenRenderStateKind as PinnedRenderInitKindWithParent<Self>>::PinnedRenderInitWithParent<R>,
    );

    fn children_pinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: Pin<&mut PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_unmounted_ui_handle: PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>;

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: Pin<&mut PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_ui_handle: &mut PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    );

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    );

    fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: &mut UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        children_unmounted_ui_handle: UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>;

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state: &mut UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        children_ui_handle: &mut UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    );
}

enum Never {}

pub struct KindRenderStateWithAnyParent<CK>(Never, std::marker::PhantomData<CK>);

impl<CK: UnpinnedRenderStateKind> UnpinnedRenderStateKind for KindRenderStateWithAnyParent<CK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = CK::UnpinnedUiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = CK::UnpinnedState<R>;
}

impl<CK: PinnedRenderStateKind> PinnedRenderStateKind for KindRenderStateWithAnyParent<CK> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = CK::PinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> = CK::PinnedState<R>;
}

impl<
        //
        CK: RenderStateKind,
        ElType: ?Sized + behavior_type_traits::Element,
    > RenderStateKindPollRenderWithParent<ElType> for KindRenderStateWithAnyParent<CK>
{
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        _: &mut <ElType as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut PinnedStateOfKind<R, Self>>,
        ui_handle: &mut PinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        CK::pinned_poll_render(renderer, state, ui_handle, cx)
    }

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        _: &mut <ElType as BehaviorType>::OfBehaviorType<R>,
        state: &mut UnpinnedStateOfKind<R, Self>,
        ui_handle: &mut UnpinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        CK::unpinned_poll_render(renderer, state, ui_handle, cx)
    }
}

impl<CK: PinnedRenderInitKind, ElType: ?Sized + behavior_type_traits::Element> PinnedRenderInitKindWithParent for KindRenderStateWithAnyParent<CK> {}

pub struct RenderInitWithAnyParent;

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderStateKind = KindRenderStateWithAnyParent<Children::RenderStateKind>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_state_default: Pin<&mut PinnedStateDefaultOfKind<R, Self::ChildrenRenderStateKind>>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.pinned_render_init(render_context, children_state_default))
    }

    fn children_pinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: Pin<&mut PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_unmounted_ui_handle: PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| {
            children.pinned_render_init_by_reusing(render_context, children_reused_state, children_unmounted_ui_handle)
        })
    }

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        _: &mut Self::OfBehaviorType<R>,
        children_state: Pin<&mut PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
        children_ui_handle: &mut PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        children.pinned_render_update(renderer, children_state, children_ui_handle)
    }

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.unpinned_render_init(render_context))
    }

    fn children_unpinned_render_init_by_reusing<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_reused_state: &mut UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        children_unmounted_ui_handle: UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) -> UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| {
            children.unpinned_render_init_by_reusing(render_context, children_reused_state, children_unmounted_ui_handle)
        })
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        _: &mut Self::OfBehaviorType<R>,
        children_state: &mut UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        children_ui_handle: &mut UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    ) {
        children.unpinned_render_update(renderer, children_state, children_ui_handle)
    }
}
