use std::{marker::PhantomData, rc::Rc};

use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::EventTarget;

use crate::render::Unset;

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
    type State: Default;

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State);
}

impl<EventType: StaticEventType> UpdateDomEventListener<EventType> for Unset {
    type State = ();

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {}
}

impl<EventType: StaticEventType, F: Fn(&EventType::Event) + ?Sized + 'static>
    UpdateDomEventListener<EventType> for EventListenerShared<EventType::Event, F>
where
    EventType::Event: JsCast,
{
    type State = Option<(Self, EventListener)>;

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        if let Some((old, _)) = state {
            if *old == self {
                return;
            }
        }

        let el = {
            let this = self.clone();
            EventListener::new(target, EventType::EVENT_TYPE, move |event| {
                (this.inner)(event.dyn_ref().expect("wrong event type"))
            })
        };

        *state = Some((self, el));
    }
}

impl<EventType: StaticEventType, F> UpdateDomEventListener<EventType> for F
where
    EventType::Event: JsCast,
    F: FnMut(&EventType::Event) + 'static,
{
    type State = Option<EventListener>;

    fn update_dom_event_listener(mut self, target: &EventTarget, state: &mut Self::State) {
        *state = Some(EventListener::new(
            target,
            EventType::EVENT_TYPE,
            move |event| self(event.dyn_ref().expect("wrong event type")),
        ));
    }
}

impl<EventType: StaticEventType, F> UpdateDomEventListener<EventType>
    for EventListenerOnce<EventType::Event, F>
where
    EventType::Event: JsCast,
    F: FnOnce(&EventType::Event) + 'static,
{
    type State = Option<EventListener>;

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        *state = Some(EventListener::once(
            target,
            EventType::EVENT_TYPE,
            move |event| (self.inner)(event.dyn_ref().expect("wrong event type")),
        ));
    }
}

impl<Event: StaticEventType, F> UpdateDomEventListener<Event> for Option<F>
where
    F: UpdateDomEventListener<Event>,
{
    type State = <F as UpdateDomEventListener<Event>>::State;

    fn update_dom_event_listener(self, target: &EventTarget, state: &mut Self::State) {
        if let Some(el) = self {
            F::update_dom_event_listener(el, target, state)
        } else {
            *state = Default::default()
        }
    }
}
