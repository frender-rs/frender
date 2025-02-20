use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Poll;

use frender_common::convert::FromMut as _;
use frender_common::reactive_value::RenderInitPinned;
use frender_dom::csr::behaviors::ElementWithChildren;

use crate::element::{
    PinnedRenderStateKind, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, RenderStateKind, UnpinnedRenderStateKind, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind,
};
use crate::{BehaviorType, RenderHtml};

use crate::html::behavior_type_traits;
use crate::CsrElement;

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

pub trait CsrComponent<Children>: BehaviorType {
    type ChildrenRenderStateKind: RenderStateKindPollRenderWithParent<Self>;
    type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized>: for<'r, 'p> RenderInitPinned<
        //
        (&'r mut R, &'p mut Self::OfBehaviorType<R>),
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        Output = PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
    >;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        parent: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        Self::ChildrenPinnedRenderInit<R>,
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

pub struct RenderInitWithAnyParent<C, T>(PhantomData<C>, T);

impl<
        //
        C: CsrComponentNormalElement,
        T: for<'a, 'b> RenderInitPinned<&'a mut R::RenderContext<'b>, S, Output = Out>,
        R: ?Sized + RenderHtml,
        S: ?Sized,
        Out,
    > RenderInitPinned<(&mut R, &mut C::OfBehaviorType<R>), S> for RenderInitWithAnyParent<C, T>
{
    type Output = Out;
    fn render_init_pinned(self, (renderer, parent): (&mut R, &mut C::OfBehaviorType<R>), state: Pin<&mut S>) -> Self::Output {
        <C::Element<R>>::from_mut(parent).with_render_context_at_first_child_of_self(
            //
            renderer,
            |render_context| self.1.render_init_pinned(render_context, state),
        )
    }
}

impl<C: CsrComponentNormalElement, Children: CsrElement> CsrComponent<Children> for C {
    type ChildrenRenderStateKind = StateKindWithAnyParent<Children::RenderStateKind>;
    type ChildrenPinnedRenderInit<R: RenderHtml + ?Sized> = RenderInitWithAnyParent<C, Children::PinnedRenderInit<R>>;

    fn children_pinned_render_init<R: RenderHtml + ?Sized>(
        //
        self,
        children: Children,
        renderer: &mut R,
        _: &mut Self::OfBehaviorType<R>,
    ) -> (
        //
        PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
        Self::ChildrenPinnedRenderInit<R>,
    ) {
        let (state, children) = children.pinned_render_init(renderer);
        (state, RenderInitWithAnyParent(PhantomData, children))
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
