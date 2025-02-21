use std::{marker::PhantomData, pin::Pin, task::Poll};

use frender_common::{
    reactive_value::{ProvideValueOfKind, ReactiveValueRenderInitPinned, ReactiveValueState, RenderInitPinned, ReusableRendererOfKind},
    value_kind::ValueKind,
};
use frender_dom::csr::{render_from::str::ValueKindForStr, StateUnmount};

use crate::{
    csr::element::{self, PinnedRenderStateKind, UnpinnedRenderStateKind},
    element_types::RenderStateKindPollRenderWithParent,
    html::{behavior_type_traits, RenderHtml},
    BehaviorType,
};

pub(crate) trait RenderKind<BT: ?Sized + BehaviorType, VK: ?Sized + ValueKind> {
    fn render<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut BT::OfBehaviorType<R>, value: VK::Value<'_>);
    fn reuse<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut BT::OfBehaviorType<R>, provide_value: impl ProvideValueOfKind<VK>);
}

pub(crate) fn render_kind_renderer<
    //
    'a,
    RK: RenderKind<BT, VK>,
    BT: ?Sized + BehaviorType,
    VK: ?Sized + ValueKind,
    R: ?Sized + RenderHtml,
>(
    renderer: &'a mut R,
    parent: &'a mut BT::OfBehaviorType<R>,
) -> impl 'a + FnMut(VK::Value<'_>) {
    |value| RK::render(renderer, parent, value)
}

pub(crate) enum Never {}

pub struct Kind<VK: ?Sized, P, U, RK>(Never, PhantomData<(P, U, RK, VK)>);

impl<VK: ?Sized, P, U: StateUnmount + Unpin, RK> UnpinnedRenderStateKind for Kind<VK, P, U, RK> {
    type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type UnpinnedState<R: RenderHtml + ?Sized> = U;
}

impl<VK: ?Sized, P: StateUnmount, U, RK> PinnedRenderStateKind for Kind<VK, P, U, RK> {
    type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
    type PinnedState<R: RenderHtml + ?Sized> = P;
}

impl<
        //
        VK: ?Sized + ValueKind,
        P: ReactiveValueState<ReactiveValueKind = VK>,
        U: Unpin + ReactiveValueState<ReactiveValueKind = VK>,
        BT: ?Sized + BehaviorType,
        RK: RenderKind<BT, VK>,
    > RenderStateKindPollRenderWithParent<BT> for Kind<VK, P, U, RK>
{
    fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut BT::OfBehaviorType<R>,
        state: Pin<&mut element::PinnedStateOfKind<R, Self>>,
        (): &mut element::PinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        ReactiveValueState::poll_render(
            //
            state,
            render_kind_renderer::<RK, BT, VK, R>(renderer, parent),
            cx,
        )
    }

    fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut BT::OfBehaviorType<R>,
        state: &mut element::UnpinnedStateOfKind<R, Self>,
        (): &mut element::UnpinnedUiHandleOfKind<R, Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()> {
        ReactiveValueState::poll_render(
            //
            Pin::new(state),
            render_kind_renderer::<RK, BT, VK, R>(renderer, parent),
            cx,
        )
    }
}

pub(crate) struct ReusableRenderer<'a, BT: BehaviorType, R: RenderHtml + ?Sized, RK> {
    pub(crate) _render_kind: PhantomData<RK>,
    pub(crate) renderer: &'a mut R,
    pub(crate) parent: &'a mut BT::OfBehaviorType<R>,
}

impl<'a, BT: BehaviorType, R: RenderHtml + ?Sized, RK> ReusableRenderer<'a, BT, R, RK> {
    pub(crate) fn new(renderer: &'a mut R, parent: &'a mut BT::OfBehaviorType<R>) -> Self {
        Self {
            _render_kind: PhantomData,
            renderer,
            parent,
        }
    }
}

impl<
        //
        VK: ?Sized + ValueKind,
        BT: BehaviorType,
        R: RenderHtml + ?Sized,
        RK: RenderKind<BT, VK>,
    > ReusableRendererOfKind<VK> for ReusableRenderer<'_, BT, R, RK>
{
    type Output = ();

    fn render(self, value: <VK as frender_common::value_kind::ValueKind>::Value<'_>) -> Self::Output {
        RK::render(self.renderer, self.parent, value)
    }

    fn reuse(self, provide_value: impl frender_common::reactive_value::ProvideValueOfKind<VK>) -> Self::Output {
        RK::reuse(self.renderer, self.parent, provide_value);
    }
}

pub struct RenderInit<BT: BehaviorType, VK: ?Sized + ValueKind, RK, T>(PhantomData<(BT, RK, VK)>, T);

impl<BT: BehaviorType, VK: ?Sized + ValueKind, RK, T> RenderInit<BT, VK, RK, T> {
    pub(crate) const fn new(init: T) -> Self {
        Self(PhantomData, init)
    }
}

impl<
        //
        BT: BehaviorType,
        VK: ?Sized + ValueKind,
        RK: RenderKind<BT, VK>,
        T: ReactiveValueRenderInitPinned<VK, S>,
        S: ?Sized,
        R: ?Sized + RenderHtml,
    > RenderInitPinned<(&mut R, &mut BT::OfBehaviorType<R>), S> for RenderInit<BT, VK, RK, T>
{
    type Output = ();
    fn render_init_pinned(self, (renderer, parent): (&mut R, &mut BT::OfBehaviorType<R>), state: Pin<&mut S>) -> Self::Output {
        let init = <T::RenderInitPinned<_, ()> as From<T>>::from(self.1);
        init.render_init_pinned(render_kind_renderer::<RK, BT, VK, R>(renderer, parent), state);
    }
}

macro_rules! impl_parent_only {
    (
        type Children = $Children:ty;

        type ValueKind = $ValueKind:ty;
        type RenderKind = $RenderKind:ty;


        const into_reactive_value: $ReactiveValue:ty =
            |$children:pat_param $(,)?| $reactive_value:expr
        ;
    ) => {
        type ChildrenRenderStateKind = crate::special::parent_only::Kind<
            //
            $ValueKind,
            <$ReactiveValue as ::frender_common::reactive_value::ReactiveValue<$ValueKind>>::PinnedState,
            <$ReactiveValue as ::frender_common::reactive_value::ReactiveValue<$ValueKind>>::UnpinnedState,
            $RenderKind,
        >;

        type ChildrenPinnedRenderInit<R: crate::html::RenderHtml + ?Sized> = crate::special::parent_only::RenderInit<
            //
            Self,
            $ValueKind,
            $RenderKind,
            <$ReactiveValue as ::frender_common::reactive_value::ReactiveValue<$ValueKind>>::PinnedRenderInit,
        >;

        fn children_pinned_render_init<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            _: &mut R,
            _: &mut <Self as crate::BehaviorType>::OfBehaviorType<R>,
        ) -> (
            //
            crate::csr::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            Self::ChildrenPinnedRenderInit<R>,
        ) {
            let (state, init) = ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::pinned_render_init($reactive_value);

            (state, crate::special::parent_only::RenderInit::new(init))
        }

        fn children_pinned_render_init_by_reusing<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            children_reused_state: ::core::pin::Pin<&mut crate::csr::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): crate::csr::element::PinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> crate::csr::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
            ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::pinned_render_init_by_reusing(
                $reactive_value,
                crate::special::parent_only::ReusableRenderer::<Self, R, $RenderKind>::new(renderer, parent),
                children_reused_state,
            );
            ()
        }

        fn children_pinned_render_update<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            children_state: ::core::pin::Pin<&mut crate::csr::element::PinnedStateOfKind<R, Self::ChildrenRenderStateKind>>,
            (): &mut crate::csr::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
            _ = ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::pinned_render_update(
                $reactive_value,
                crate::special::parent_only::render_kind_renderer::<$RenderKind, Self, $ValueKind, R>(renderer, parent),
                children_state,
            )
        }

        fn children_unpinned_render_init<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
        ) -> (
            //
            crate::csr::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            crate::csr::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
            ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::unpinned_render_init(
                //
                $reactive_value,
                crate::special::parent_only::render_kind_renderer::<$RenderKind, Self, $ValueKind, R>(renderer, parent),
            )
        }

        fn children_unpinned_render_init_by_reusing<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            children_reused_state: &mut crate::csr::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): crate::csr::element::UnpinnedUnmountedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) -> crate::csr::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
            ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::unpinned_render_init_by_reusing(
                $reactive_value,
                crate::special::parent_only::ReusableRenderer::<Self, R, $RenderKind>::new(renderer, parent),
                children_reused_state,
            );
            ()
        }

        fn children_unpinned_render_update<R: crate::html::RenderHtml + ?Sized>(
            //
            self,
            $children: $Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            children_state: &mut crate::csr::element::UnpinnedStateOfKind<R, Self::ChildrenRenderStateKind>,
            (): &mut crate::csr::element::UnpinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind>,
        ) {
            _ = ::frender_common::reactive_value::ReactiveValue::<$ValueKind>::unpinned_render_update(
                $reactive_value,
                crate::special::parent_only::render_kind_renderer::<$RenderKind, Self, $ValueKind, R>(renderer, parent),
                children_state,
            )
        }
    };
}

pub(crate) use impl_parent_only;

pub enum RenderInnerTextKind {}

impl<BT: behavior_type_traits::HtmlElement, VK: ?Sized + ValueKindForStr> RenderKind<BT, VK> for RenderInnerTextKind {
    fn render<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>, value: <VK as frender_common::value_kind::ValueKind>::Value<'_>) {
        use frender_common::convert::IntoMut as _;
        use frender_dom::csr::behaviors::SetInnerTextFromStr as _;

        let parent: &mut BT::HtmlElement<R> = parent.into_mut();
        parent.set_inner_text_from_str(renderer, value)
    }

    fn reuse<R: RenderHtml + ?Sized>(renderer: &mut R, parent: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>, provide_value: impl frender_common::reactive_value::ProvideValueOfKind<VK>) {
        // TODO: check when debug_assertions
        let _ = (renderer, parent, provide_value);
    }
}
