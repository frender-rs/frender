use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

use frender_common::convert::FromMut as _;
use frender_dom::behaviors::ElementWithChildren;
use frender_dom::render::RenderWithContext;

use crate::element::{
    CsrElementRenderInitPinned, PinnedRenderInitKind, PinnedRenderStateKind, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, RenderStateKind, UnpinnedRenderStateKind, UnpinnedStateOfKind,
    UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind,
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

pub trait RenderInitPinnedWithParent<ParentType: ?Sized + BehaviorType, R: ?Sized + RenderHtml> {
    type UiHandle;
    type State;
    fn render_init_pinned_with_parent(
        //
        self,
        renderer: &mut R,
        parent: &mut ParentType::OfBehaviorType<R>,
        children_state: Pin<&mut Self::State>,
    ) -> Self::UiHandle;
}

pub trait PinnedRenderInitKindWithParent<ParentType: ?Sized + BehaviorType> {
    type PinnedRenderStateKind: PinnedRenderStateKind;
    type PinnedRenderInitWithParent<R: ?Sized + RenderHtml>: RenderInitPinnedWithParent<
        ParentType,
        R,
        UiHandle = PinnedUiHandleOfKind<R, Self::PinnedRenderStateKind>,
        State = PinnedStateOfKind<R, Self::PinnedRenderStateKind>,
    >;
}

pub trait CsrComponent<Children>: BehaviorType {
    type ChildrenRenderStateKind: RenderStateKindPollRenderWithParent<Self>;
    type ChildrenRenderInitKind: PinnedRenderInitKindWithParent<Self>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
    ) -> (
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        <Self::ChildrenRenderInitKind as PinnedRenderInitKindWithParent<Self>>::PinnedRenderInitWithParent<R>,
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

pub struct StateKindWithAnyParent<CK>(Never, PhantomData<CK>);
pub struct InitKindWithAnyParent<CK>(Never, PhantomData<CK>);

impl<CK: UnpinnedRenderStateKind> UnpinnedRenderStateKind for StateKindWithAnyParent<CK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = CK::UnpinnedUiHandle<R>;
    type UnpinnedState<R: RenderHtml + ?Sized> = CK::UnpinnedState<R>;
}

impl<CK: PinnedRenderStateKind> PinnedRenderStateKind for StateKindWithAnyParent<CK> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = CK::PinnedUiHandle<R>;
    type PinnedState<R: RenderHtml + ?Sized> = CK::PinnedState<R>;
}

impl<
        //
        CK: RenderStateKind,
        ElType: ?Sized + behavior_type_traits::Element,
    > RenderStateKindPollRenderWithParent<ElType> for StateKindWithAnyParent<CK>
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

impl<CK: PinnedRenderInitKind, ElType: ?Sized + behavior_type_traits::Element> PinnedRenderInitKindWithParent<ElType> for InitKindWithAnyParent<CK> {
    type PinnedRenderStateKind = StateKindWithAnyParent<CK::PinnedRenderStateKind>;
    type PinnedRenderInitWithParent<R: ?Sized + RenderHtml> = RenderInitWithAnyParent<CK::PinnedRenderInit<R>>;
}

pub struct RenderInitWithAnyParent<T>(T);

impl<T: CsrElementRenderInitPinned<R>, ElType: ?Sized + behavior_type_traits::Element, R: ?Sized + RenderHtml> RenderInitPinnedWithParent<ElType, R> for RenderInitWithAnyParent<T> {
    type UiHandle = T::UiHandle;
    type State = T::State;

    fn render_init_pinned_with_parent(
        //
        self,
        renderer: &mut R,
        parent: &mut <ElType as BehaviorType>::OfBehaviorType<R>,
        children_state: Pin<&mut Self::State>,
    ) -> Self::UiHandle {
        <ElType::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(
            //
            renderer,
            |render_context| self.0.render_init_pinned(render_context, children_state),
        )
    }
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderStateKind = StateKindWithAnyParent<Children::RenderStateKind>;
    type ChildrenRenderInitKind = InitKindWithAnyParent<Children::RenderInitKind>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
    ) -> (
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        <Self::ChildrenRenderInitKind as PinnedRenderInitKindWithParent<Self>>::PinnedRenderInitWithParent<R>,
    ) {
        let (state, render_init) = children.pinned_render_init(renderer);
        (state, RenderInitWithAnyParent(render_init))
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
