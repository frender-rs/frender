use frender_common::HandleEvent;
use frender_csr::event_listener::{RegisterOrUpdate, RegisterUpdate};

use crate::event_types::EventType;

pub trait OnEvent<Renderer: ?Sized, ET: EventType> {
    type EventListener<F: HandleEvent<ET::Event> + 'static>: Default
        + 'static
        + RegisterOrUpdate<Self, Renderer, F>;

    type EventListenerUnpinned<F: HandleEvent<ET::Event> + 'static>: 'static
        + RegisterUpdate<Self, Renderer, F>;
}
