use std::{marker::PhantomData, pin::Pin};

use frender_common::{convert::IdentityAs, reactive_value::RenderInitPinned};
use frender_dom::{
    csr::{OnEvent, ProvideMutMounted, UiHandle},
    event_types::EventType,
};

use crate::{
    csr::element::HtmlRenderContext,
    csr::kinds::{KindOfNoState, RenderInitNothing},
    html::RenderHtml,
    into_property::IntoProperty,
};

pub trait BehaviorType {
    type OfBehaviorType<Renderer: ?Sized + RenderHtml>;
}

pub trait UiHandleType: BehaviorType {
    type UnmountedUiHandle<Renderer: ?Sized + RenderHtml>: ProvideMutMounted<Renderer, Mounted = Self::UiHandle<Renderer>>;
    type UiHandle<Renderer: ?Sized + RenderHtml>: UiHandle<Renderer, Unmounted = Self::UnmountedUiHandle<Renderer>> + IdentityAs<Self::OfBehaviorType<Renderer>>;

    fn create_unmounted_ui_handle_of_type<R: ?Sized + RenderHtml>(renderer: &mut R) -> Self::UnmountedUiHandle<R>;

    fn create_and_mount_ui_handle_of_type<Ctx: ?Sized + HtmlRenderContext>(render_context: &mut Ctx) -> Self::UiHandle<Ctx::Renderer> {
        use frender_dom::csr::{render::RenderContext as _, UnmountedUiHandle as _};
        render_context.map_mut_render_context(|render_context| {
            Self::create_unmounted_ui_handle_of_type(render_context.renderer_mut())
                //
                .mount(render_context)
        })
    }
}

pub trait OnEventType<EVT: EventType>: BehaviorType {
    type OnEvent<Renderer: ?Sized + RenderHtml>: OnEvent<Renderer, EVT> + IdentityAs<Self::OfBehaviorType<Renderer>>;
}

pub trait UnpinnedNonReactiveRenderStateKind {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml>;
}

pub trait PinnedNonReactiveRenderStateKind {
    type PinnedNonReactiveState<R: ?Sized + RenderHtml>;
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
    type PinnedRenderInitWithBehavior<R: ?Sized + RenderHtml>: for<'r, 'b> RenderInitPinned<
        (&'r mut R, &'b mut BT::OfBehaviorType<R>),
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Output = (),
    >;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
    ) -> (
        //
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Self::PinnedRenderInitWithBehavior<R>,
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
    type PinnedRenderInitWithBehavior<R: ?Sized + RenderHtml> = RenderInitNothing;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (): Self,
        _: &mut R,
        _: &mut BT::OfBehaviorType<R>,
    ) -> (
        //
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Self::PinnedRenderInitWithBehavior<R>,
    ) {
        ((), RenderInitNothing)
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

pub struct RenderInitTwo<A, B>(A, B);

impl<
        //
        A: for<'a, 'b> RenderInitPinned<(&'a mut R1, &'b mut R2), SA, Output = ()>,
        B: for<'a, 'b> RenderInitPinned<(&'a mut R1, &'b mut R2), SB, Output = ()>,
        R1: ?Sized,
        R2: ?Sized,
        SA,
        SB,
    > RenderInitPinned<(&mut R1, &mut R2), TwoStates<SA, SB>> for RenderInitTwo<A, B>
{
    type Output = ();

    fn render_init_pinned(self, (r1, r2): (&mut R1, &mut R2), state: Pin<&mut TwoStates<SA, SB>>) -> Self::Output {
        let state = state.project();
        self.0.render_init_pinned((r1, r2), state.a);
        self.1.render_init_pinned((r1, r2), state.b);
    }
}

impl<BT: BehaviorType, A: PinnedRenderWithBehavior<BT>, B: PinnedRenderWithBehavior<BT>> PinnedRenderWithBehavior<BT> for (A, B) {
    type PinnedRenderStateKind = KindOfTwo<A::PinnedRenderStateKind, B::PinnedRenderStateKind>;
    type PinnedRenderInitWithBehavior<R: ?Sized + RenderHtml> = RenderInitTwo<
        //
        A::PinnedRenderInitWithBehavior<R>,
        B::PinnedRenderInitWithBehavior<R>,
    >;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        (this_a, this_b): Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
    ) -> (
        //
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Self::PinnedRenderInitWithBehavior<R>,
    ) {
        let (a, render_init_a) = A::pinned_render_init_with_behavior(this_a, renderer, b);
        let (b, render_init_b) = B::pinned_render_init_with_behavior(this_b, renderer, b);
        (TwoStates { a, b }, RenderInitTwo(render_init_a, render_init_b))
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
    type PinnedRenderInitWithBehavior<R: ?Sized + RenderHtml> = <T::IntoProperty as PinnedRenderWithBehavior<BT>>::PinnedRenderInitWithBehavior<R>;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
    ) -> (
        //
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Self::PinnedRenderInitWithBehavior<R>,
    ) {
        <T::IntoProperty>::pinned_render_init_with_behavior(T::into_property(this), renderer, b)
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
