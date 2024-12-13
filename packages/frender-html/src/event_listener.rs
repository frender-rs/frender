use std::{marker::PhantomData, pin::Pin};

use frender_common::convert::FromMut;
use frender_dom::{event_types::EventType, HandleEvent, MaybeHandleEvent};

use crate::{
    update_element::{OnEventType, PinnedNonReactiveRenderStateKind, PinnedRenderWithBehavior, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    RenderHtml,
};

pub type UnpinnedEventListenerOf<EVT, E, R, F> = <E as ::frender_dom::OnEvent<R, EVT>>::EventListenerUnpinned<F>;
pub type PinnedEventListenerOf<EVT, E, R, F> = <E as ::frender_dom::OnEvent<R, EVT>>::EventListener<F>;

enum Never {}
pub struct Kind<EVT, ET, H>(Never, PhantomData<(EVT, ET, H)>);

impl<EVT: EventType, ET: OnEventType<EVT>, H: HandleEvent<EVT::Event> + 'static> UnpinnedNonReactiveRenderStateKind for Kind<EVT, ET, H> {
    type UnpinnedNonReactiveState<R: ?Sized + crate::RenderHtml> = UnpinnedEventListenerOf<EVT, ET::OnEvent<R>, R, H>;
}

impl<EVT: EventType, ET: OnEventType<EVT>, F: HandleEvent<EVT::Event> + 'static> PinnedNonReactiveRenderStateKind for Kind<EVT, ET, F> {
    type PinnedNonReactiveState<R: ?Sized + crate::RenderHtml> = PinnedEventListenerOf<EVT, ET::OnEvent<R>, R, F>;
}

pub struct Property<EVT, F> {
    _event_type: PhantomData<EVT>,
    f: F,
}

impl<EVT, F> Property<EVT, F> {
    pub(crate) fn new(f: F) -> Self {
        Self { _event_type: PhantomData, f }
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

    fn pinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        state: ::core::pin::Pin<&mut <Self::PinnedRenderStateKind as crate::update_element::PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        <Self as PinnedRenderWithBehavior<BT>>::pinned_render_update_with_behavior(this, renderer, b, state)
    }

    fn pinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut BT::OfBehaviorType<R>,
        mut state: ::core::pin::Pin<&mut <Self::PinnedRenderStateKind as crate::update_element::PinnedNonReactiveRenderStateKind>::PinnedNonReactiveState<R>>,
    ) {
        let element = <BT::OnEvent<R>>::from_mut(b);

        if let Some(this) = this.f.into() {
            frender_dom::RegisterOrUpdate::register_or_update(state, element, renderer, this)
        } else {
            state.set(Default::default())
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
        // TODO: refactor without default
        let mut state = Default::default();
        <Self as UnpinnedRenderWithBehavior<BT>>::unpinned_render_update_with_behavior(this, renderer, b, &mut state);
        state
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        let element = <BT::OnEvent<R>>::from_mut(b);
        let mut state = Pin::new(state);

        if let Some(this) = this.f.into() {
            frender_dom::RegisterOrUpdate::register_or_update(state, element, renderer, this)
        } else {
            state.set(Default::default())
        }
    }
}
