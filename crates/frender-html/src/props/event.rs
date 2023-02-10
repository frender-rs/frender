use std::{marker::PhantomData, rc::Rc};

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;

pub trait StaticEventType {
    const EVENT_TYPE: &'static str;

    type Event;
}

pub struct EventListenerShared<Event, F: Fn(&Event) + ?Sized> {
    inner: Rc<F>,
    _event: PhantomData<Event>,
}

impl<Event, F: Fn(&Event) + ?Sized> Unpin for EventListenerShared<Event, F> {}

impl<Event, F: Fn(&Event) + ?Sized> Clone for EventListenerShared<Event, F> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _event: PhantomData,
        }
    }
}

impl<Event, F: Fn(&Event) + ?Sized> PartialEq for EventListenerShared<Event, F> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<Event, F: Fn(&Event) + ?Sized> EventListenerShared<Event, F> {
    #[inline]
    pub fn new(f: Rc<F>) -> Self {
        Self {
            inner: f,
            _event: PhantomData,
        }
    }
}

pub type EventListenerSharedDyn<Event> = EventListenerShared<Event, dyn Fn(&Event)>;

pub struct EventListenerOnce<Event, F: FnOnce(&Event)> {
    inner: F,
    _event: PhantomData<Event>,
}

impl<Event, F: FnOnce(&Event)> EventListenerOnce<Event, F> {
    #[inline]
    pub fn new(f: F) -> Self {
        Self {
            inner: f,
            _event: PhantomData,
        }
    }
}

pub type EventListenerOnceDyn<Event> = EventListenerOnce<Event, Box<dyn FnOnce(&Event)>>;

pub trait UpdateDomEventListener<EventType: StaticEventType>: Sized {
    type State: 'static;

    fn initialize_dom_event_listener_state(self, target: &EventTarget) -> Self::State;

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        *state = <Self as UpdateDomEventListener<EventType>>::initialize_dom_event_listener_state(
            self, target,
        );
    }
}

impl<EventType: StaticEventType> UpdateDomEventListener<EventType> for () {
    type State = ();

    #[inline(always)]
    fn initialize_dom_event_listener_state(self, _: &EventTarget) -> Self::State {}

    #[inline(always)]
    fn update_dom_event_listener(self, _: &EventTarget, _: &mut Self::State) {}
}

impl<EventType: StaticEventType, F: Fn(&EventType::Event) + ?Sized + 'static>
    UpdateDomEventListener<EventType> for EventListenerShared<EventType::Event, F>
where
    EventType::Event: JsCast + 'static,
{
    type State = (Self, EventListener);

    fn initialize_dom_event_listener_state(self, target: &EventTarget) -> Self::State {
        let el = {
            let this = self.clone();
            EventListener::new(target, EventType::EVENT_TYPE, move |event| {
                (this.inner)(event.dyn_ref().expect("wrong event type"))
            })
        };
        (self, el)
    }

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        if state.0 == self {
            return;
        }

        *state = <Self as UpdateDomEventListener<EventType>>::initialize_dom_event_listener_state(
            self, target,
        );
    }
}

impl<EventType: StaticEventType, F> UpdateDomEventListener<EventType> for F
where
    EventType::Event: JsCast,
    F: FnMut(&EventType::Event) + 'static,
{
    type State = EventListener;

    fn initialize_dom_event_listener_state(mut self, target: &EventTarget) -> Self::State {
        EventListener::new(target, EventType::EVENT_TYPE, move |event| {
            self(event.dyn_ref().expect("wrong event type"))
        })
    }
}

impl<EventType: StaticEventType, F> UpdateDomEventListener<EventType>
    for EventListenerOnce<EventType::Event, F>
where
    EventType::Event: JsCast,
    F: FnOnce(&EventType::Event) + 'static,
{
    type State = EventListener;

    fn initialize_dom_event_listener_state(self, target: &EventTarget) -> Self::State {
        EventListener::once(target, EventType::EVENT_TYPE, move |event| {
            (self.inner)(event.dyn_ref().expect("wrong event type"))
        })
    }
}

impl<Event: StaticEventType, F> UpdateDomEventListener<Event> for Option<F>
where
    F: UpdateDomEventListener<Event>,
{
    type State = Option<<F as UpdateDomEventListener<Event>>::State>;

    fn initialize_dom_event_listener_state(self, target: &EventTarget) -> Self::State {
        if let Some(el) = self {
            Some(F::initialize_dom_event_listener_state(el, target))
        } else {
            None
        }
    }

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        match (self, state) {
            (Some(el), Some(state)) => el.update_dom_event_listener(target, state),
            (Some(el), state @ None) => {
                *state = Some(el.initialize_dom_event_listener_state(target))
            }
            (None, state) => {
                *state = None;
            }
        }
    }
}
