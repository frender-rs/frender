use std::{marker::PhantomData, pin::Pin};

use frender_common::convert::IdentityAs;
use frender_dom::event_types::EventType;

use crate::{kinds::KindOfNoState, HtmlRenderContext, RenderHtml};

pub trait BehaviorType {
    type OfBehaviorType<Renderer: ?Sized + RenderHtml>;
}

pub trait UiHandleType: BehaviorType {
    type UiHandle<Renderer: ?Sized + RenderHtml>: IdentityAs<Self::OfBehaviorType<Renderer>>;

    fn create_and_mount_ui_handle_of_type<Ctx: ?Sized + HtmlRenderContext>(render_context: &mut Ctx) -> Self::UiHandle<Ctx::Renderer>;
}

pub trait OnEventType<EVT: EventType>: BehaviorType {
    type OnEvent<Renderer: ?Sized + RenderHtml>: frender_dom::OnEvent<Renderer, EVT> + IdentityAs<Self::OfBehaviorType<Renderer>>;
}

pub trait UnpinnedNonReactiveRenderStateKind {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml>;
}

pub trait PinnedNonReactiveRenderStateKind {
    type PinnedNonReactiveState<R: ?Sized + RenderHtml>: Default;
}

pub trait UnpinnedRenderWithBehavior<BT: BehaviorType> {
    type UnpinnedRenderStateKind: UnpinnedNonReactiveRenderStateKind;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>;

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    );
}

pub trait PinnedRenderWithBehavior<BT: BehaviorType> {
    type PinnedRenderStateKind: PinnedNonReactiveRenderStateKind;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    );

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    );
}

// region: for ()

impl UnpinnedNonReactiveRenderStateKind for KindOfNoState {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = ();
}

impl<BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for () {
    type UnpinnedRenderStateKind = KindOfNoState;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (): Self,
        _: &mut R,
        _: &mut <BT as BehaviorType>::OfBehaviorType<R>,
    ) {
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        (): Self,
        _: &mut R,
        _: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        (): &mut (),
    ) {
    }
}

impl PinnedNonReactiveRenderStateKind for KindOfNoState {
    type PinnedNonReactiveState<R: ?Sized + RenderHtml> = ();
}

impl<BT: BehaviorType> PinnedRenderWithBehavior<BT> for () {
    type PinnedRenderStateKind = KindOfNoState;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (): Self,
        _: &mut R,
        _: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        _: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
    }

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        (): Self,
        _: &mut R,
        _: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        _: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
    }
}

// endregion
// region: for (A, B)

enum Never {}
pub struct KindOfTwo<AK, BK>(Never, PhantomData<(AK, BK)>);

impl<AK: UnpinnedNonReactiveRenderStateKind, BK: UnpinnedNonReactiveRenderStateKind> UnpinnedNonReactiveRenderStateKind for KindOfTwo<AK, BK> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = (AK::UnpinnedNonReactiveState<R>, BK::UnpinnedNonReactiveState<R>);
}
impl<AK: PinnedNonReactiveRenderStateKind, BK: PinnedNonReactiveRenderStateKind> PinnedNonReactiveRenderStateKind for KindOfTwo<AK, BK> {
    type PinnedNonReactiveState<R: ?Sized + RenderHtml> = TwoStates<AK::PinnedNonReactiveState<R>, BK::PinnedNonReactiveState<R>>;
}

pin_project_lite::pin_project!(
    #[derive(Default)]
    pub struct TwoStates<A, B> {
        #[pin]
        a: A,
        #[pin]
        b: B,
    }
);

impl<BT: BehaviorType, A: UnpinnedRenderWithBehavior<BT>, B: UnpinnedRenderWithBehavior<BT>> UnpinnedRenderWithBehavior<BT> for (A, B) {
    type UnpinnedRenderStateKind = KindOfTwo<A::UnpinnedRenderStateKind, B::UnpinnedRenderStateKind>;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (this_a, this_b): Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        (
            //
            A::unpinned_render_init_with_behavior(this_a, renderer, b),
            B::unpinned_render_init_with_behavior(this_b, renderer, b),
        )
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        (this_a, this_b): Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        (state_a, state_b): &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        A::unpinned_render_update_with_behavior(this_a, renderer, b, state_a);
        B::unpinned_render_update_with_behavior(this_b, renderer, b, state_b);
    }
}

impl<BT: BehaviorType, A: PinnedRenderWithBehavior<BT>, B: PinnedRenderWithBehavior<BT>> PinnedRenderWithBehavior<BT> for (A, B) {
    type PinnedRenderStateKind = KindOfTwo<A::PinnedRenderStateKind, B::PinnedRenderStateKind>;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (this_a, this_b): Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        let state = state.project();
        A::pinned_render_init_with_behavior(this_a, renderer, b, state.a);
        B::pinned_render_init_with_behavior(this_b, renderer, b, state.b);
    }

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        (this_a, this_b): Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        let state = state.project();
        A::pinned_render_update_with_behavior(this_a, renderer, b, state.a);
        B::pinned_render_update_with_behavior(this_b, renderer, b, state.b);
    }
}
// endregion
// region: Into

pub trait IntoProperty {
    type IntoProperty;
    fn into_property(this: Self) -> Self::IntoProperty;
}

impl<T: IntoProperty, BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for T
where
    T::IntoProperty: UnpinnedRenderWithBehavior<BT>,
{
    type UnpinnedRenderStateKind = <T::IntoProperty as UnpinnedRenderWithBehavior<BT>>::UnpinnedRenderStateKind;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        <T::IntoProperty>::unpinned_render_init_with_behavior(T::into_property(this), renderer, b)
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        <T::IntoProperty>::unpinned_render_update_with_behavior(T::into_property(this), renderer, b, state)
    }
}

impl<T: IntoProperty, BT: BehaviorType> PinnedRenderWithBehavior<BT> for T
where
    //
    T::IntoProperty: PinnedRenderWithBehavior<BT>,
{
    type PinnedRenderStateKind = <T::IntoProperty as PinnedRenderWithBehavior<BT>>::PinnedRenderStateKind;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        <T::IntoProperty>::pinned_render_init_with_behavior(T::into_property(this), renderer, b, state)
    }

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: Pin<&mut <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        <T::IntoProperty>::pinned_render_update_with_behavior(T::into_property(this), renderer, b, state)
    }
}

// endregion
