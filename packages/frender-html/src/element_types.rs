use std::pin::Pin;
use std::task::Poll;

use frender_common::convert::FromMut as _;
use frender_dom::behaviors::ElementWithChildren;

use crate::element::{
    PinMutRenderInitStatesOfKind, PinnedMutRenderStatesOfKind, PinnedRenderStateKind, PinnedRenderStateKindPollRender, PinnedUiHandleOfKind, UnpinnedMutRenderStatesOfKind, UnpinnedRenderStateKind,
    UnpinnedRenderStateKindPollRender, UnpinnedRenderStatesOfKind,
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

pub trait CsrComponent<Children>: BehaviorType {
    type ChildrenRenderStateKind: RenderStateKindPollRenderWithParent<Self>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>;

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    );

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R>;

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    );
}

enum Never {}

pub struct KindRenderStateWithAnyParent<CK: UnpinnedRenderStateKind + PinnedRenderStateKind>(Never, std::marker::PhantomData<CK>);

impl<CK: UnpinnedRenderStateKind + PinnedRenderStateKind> UnpinnedRenderStateKind for KindRenderStateWithAnyParent<CK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = CK::UnpinnedUiHandle<R>;
    type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = CK::UnpinnedNonReactiveState<R>;
    type UnpinnedReactiveState = CK::UnpinnedReactiveState;
}

impl<CK: UnpinnedRenderStateKind + PinnedRenderStateKind> PinnedRenderStateKind for KindRenderStateWithAnyParent<CK> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = CK::PinnedUiHandle<R>;
    type PinnedNonReactiveState<R: RenderHtml + ?Sized> = CK::PinnedNonReactiveState<R>;
    type PinnedReactiveState = CK::PinnedReactiveState;
}

impl<CK: UnpinnedRenderStateKindPollRender + PinnedRenderStateKindPollRender, ElType: ?Sized + behavior_type_traits::Element> RenderStateKindPollRenderWithParent<ElType> for KindRenderStateWithAnyParent<CK> {
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        _: &mut <ElType as BehaviorType>::OfBehaviorType<R>,
        states: PinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        CK::pinned_poll_render(renderer, states, cx)
    }
    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        _: &mut <ElType as BehaviorType>::OfBehaviorType<R>,
        states: UnpinnedMutRenderStatesOfKind<Self, R>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        CK::unpinned_poll_render(renderer, states, cx)
    }
}

impl<C: CsrComponentNormalElement, Children: Element> CsrComponent<Children> for C {
    type ChildrenRenderStateKind = KindRenderStateWithAnyParent<Children::RenderStateKind>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) -> PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.pinned_render_init(render_context, children_states))
    }

    fn children_pinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.pinned_render_update(render_context, children_states))
    }

    fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R> {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.unpinned_render_init(render_context))
    }

    fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
        children_states: UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
    ) {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(renderer, |render_context| children.unpinned_render_update(render_context, children_states))
    }
}
