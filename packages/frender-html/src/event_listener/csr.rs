use std::{marker::PhantomData, pin::Pin};

use frender_common::{convert::FromMut, reactive_value::RenderInitPinned};
use frender_dom::{event_types::EventType, HandleEvent, MaybeHandleEvent, PinnedRegisterUpdate, RegisterUpdate};

use crate::{
    update_element::{OnEventType, PinnedNonReactiveRenderStateKind, PinnedRenderWithBehavior, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    RenderHtml,
};

use super::Property;

pub type UnpinnedEventListenerOf<EVT, E, R, F> = <E as ::frender_dom::OnEvent<R, EVT>>::EventListenerUnpinned<F>;
pub type PinnedEventListenerOf<EVT, E, R, F> = <E as ::frender_dom::OnEvent<R, EVT>>::EventListener<F>;

enum Never {}
pub struct Kind<EVT, ET, H>(Never, PhantomData<(EVT, ET, H)>);

impl<EVT: EventType, ET: OnEventType<EVT>, H: HandleEvent<EVT::Event> + 'static> UnpinnedNonReactiveRenderStateKind for Kind<EVT, ET, H> {
    type UnpinnedNonReactiveState<R: ?Sized + crate::RenderHtml> = Option<UnpinnedEventListenerOf<EVT, ET::OnEvent<R>, R, H>>;
}

impl<EVT: EventType, ET: OnEventType<EVT>, F: HandleEvent<EVT::Event> + 'static> PinnedNonReactiveRenderStateKind for Kind<EVT, ET, F> {
    type PinnedNonReactiveState<R: ?Sized + crate::RenderHtml> = Option<PinnedEventListenerOf<EVT, ET::OnEvent<R>, R, F>>;
}

pub struct RenderInit<EVT, BT, T>(PhantomData<(EVT, BT)>, Option<T>);

impl<
        //
        EVT: EventType,
        BT: OnEventType<EVT>,
        T: for<'n, 'r> RenderInitPinned<(&'n mut BT::OnEvent<R>, &'r mut R), EL, Output = ()>,
        EL,
        R: ?Sized + RenderHtml,
    > RenderInitPinned<(&mut R, &mut BT::OfBehaviorType<R>), Option<EL>> for RenderInit<EVT, BT, T>
{
    type Output = ();
    fn render_init_pinned(self, (renderer, b): (&mut R, &mut BT::OfBehaviorType<R>), state: Pin<&mut Option<EL>>) -> Self::Output {
        match (self.1, state.as_pin_mut()) {
            (None, None) => {}
            (Some(init), Some(state)) => {
                let element = <BT::OnEvent<R>>::from_mut(b);
                init.render_init_pinned((element, renderer), state)
            }
            _ => unreachable!(),
        }
    }
}

impl<
        //
        EVT: EventType,
        H: HandleEvent<EVT::Event> + 'static,
        F: MaybeHandleEvent<EVT::Event, HandleEvent = H> + 'static,
        BT: OnEventType<EVT>,
    > PinnedRenderWithBehavior<BT> for Property<EVT, F>
{
    type PinnedRenderStateKind = Kind<EVT, BT, H>;
    type PinnedRenderInitWithBehavior<R: ?Sized + RenderHtml> = RenderInit<
        //
        EVT,
        BT,
        <PinnedEventListenerOf<
            //
            EVT,
            BT::OnEvent<R>,
            R,
            H,
        > as PinnedRegisterUpdate<BT::OnEvent<R>, R, H>>::PinnedRegisterInit,
    >;

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> (
        //
        <Self::PinnedRenderStateKind as PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>,
        Self::PinnedRenderInitWithBehavior<R>,
    ) {
        if let Some(this) = this.f.into() {
            let element = <BT::OnEvent<R>>::from_mut(b);
            let (state, init) = PinnedRegisterUpdate::pinned_register_init(element, renderer, this);
            (Some(state), RenderInit(PhantomData, Some(init)))
        } else {
            (None, RenderInit(PhantomData, None))
        }
    }

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        mut state: ::core::pin::Pin<&mut <Self::PinnedRenderStateKind as crate::update_element::PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        if let Some(this) = this.f.into() {
            let element = <BT::OnEvent<R>>::from_mut(b);
            if let Some(state) = state.as_mut().as_pin_mut() {
                PinnedRegisterUpdate::pinned_update(state, element, renderer, this)
            } else {
                let (init_state, init) = PinnedRegisterUpdate::pinned_register_init(element, renderer, this);
                state.set(Some(init_state));
                init.render_init_pinned((element, renderer), state.as_pin_mut().unwrap());
            }
        } else {
            state.set(None)
        }
    }
}

impl<
        //
        EVT: EventType,
        H: HandleEvent<EVT::Event> + 'static,
        F: MaybeHandleEvent<EVT::Event, HandleEvent = H> + 'static,
        BT: OnEventType<EVT>,
    > UnpinnedRenderWithBehavior<BT> for Property<EVT, F>
{
    type UnpinnedRenderStateKind = Kind<EVT, BT, H>;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        let node = <BT::OnEvent<R>>::from_mut(b);
        if let Some(f) = this.f.into() {
            Some(RegisterUpdate::register(node, renderer, f))
        } else {
            None
        }
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        let node = <BT::OnEvent<R>>::from_mut(b);

        if let Some(f) = this.f.into() {
            if let Some(state) = state {
                RegisterUpdate::update(state, node, renderer, f)
            } else {
                *state = Some(RegisterUpdate::register(node, renderer, f))
            }
        } else {
            // drop the event listener if there was some
            *state = None
        }
    }
}
