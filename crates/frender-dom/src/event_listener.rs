use frender_common::HandleEvent;
use frender_csr::event_listener::RegisterOrUpdate;

use crate::event_types::EventType;

pub trait OnEvent<Renderer: ?Sized, ET: EventType> {
    type EventListener<F: HandleEvent<ET::Event> + 'static>: Default
        + 'static
        + RegisterOrUpdate<Self, Renderer, F>;

    type EventListenerUnpinned<F: HandleEvent<ET::Event> + 'static>: Default
        + Unpin
        + 'static
        + RegisterOrUpdate<Self, Renderer, F>;
}

pub enum NoEvent {}

impl frender_events::event::Event for NoEvent {
    fn type_(&self) -> std::borrow::Cow<str> {
        match *self {}
    }

    fn event_phase(&self) -> u16 {
        match *self {}
    }

    fn bubbles(&self) -> bool {
        match *self {}
    }

    fn cancelable(&self) -> bool {
        match *self {}
    }

    fn default_prevented(&self) -> bool {
        match *self {}
    }

    fn composed(&self) -> bool {
        match *self {}
    }

    fn is_trusted(&self) -> bool {
        match *self {}
    }

    fn time_stamp(&self) -> f64 {
        match *self {}
    }

    fn cancel_bubble(&self) -> bool {
        match *self {}
    }

    fn set_cancel_bubble(&self, _: bool) {
        match *self {}
    }

    fn prevent_default(&self) {
        match *self {}
    }

    fn stop_immediate_propagation(&self) {
        match *self {}
    }

    fn stop_propagation(&self) {
        match *self {}
    }

    fn target_form_control_value(&self) -> Option<std::borrow::Cow<str>> {
        match *self {}
    }

    fn target_form_control_value_is_empty(&self) -> Option<bool> {
        match *self {}
    }

    fn set_target_form_control_default_value(&self, _: &str) -> bool {
        match *self {}
    }

    fn set_target_form_control_value(&self, _: &str) -> bool {
        match *self {}
    }

    fn target_input_value_as_number(&self) -> Option<f64> {
        match *self {}
    }

    fn target_input_checked(&self) -> Option<bool> {
        match *self {}
    }
}

pub enum NoEventType {}

impl EventType for NoEventType {
    type Event = NoEvent;
}

#[derive(Debug, Default)]
pub struct NoEventListener;

impl<N: ?Sized, R: ?Sized, F> RegisterOrUpdate<N, R, F> for NoEventListener {
    fn register_or_update(self: std::pin::Pin<&mut Self>, _: &mut N, _: &mut R, _: F) {
        // Do nothing
    }
}

impl<N: ?Sized, Renderer: ?Sized> OnEvent<Renderer, NoEventType> for N {
    type EventListener<F: HandleEvent<<NoEventType as EventType>::Event> + 'static> =
        NoEventListener;

    type EventListenerUnpinned<F: HandleEvent<<NoEventType as EventType>::Event> + 'static> =
        NoEventListener;
}
