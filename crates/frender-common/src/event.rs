/// The default value is the same as documented in
/// [gloo_events](https://docs.rs/gloo-events/0.2.0/gloo_events/struct.EventListenerOptions.html#default).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventListenerOptions {
    pub capture: bool,
    pub passive: bool,
}

impl EventListenerOptions {
    pub const DEFAULT: Self = Self {
        capture: false,
        passive: true,
    };

    pub fn is_default(&self) -> bool {
        *self == Self::DEFAULT
    }
}

impl Default for EventListenerOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// This trait exists because we can't impl [`FnMut`] on stable rust.
pub trait HandleEvent<E: ?Sized> {
    fn handle_event(&mut self, event: &E);

    /// Only used when this handler is registered.
    fn event_listener_options(&self) -> EventListenerOptions {
        Default::default()
    }
}

impl<E: ?Sized, F: FnMut(&E)> HandleEvent<E> for F {
    fn handle_event(&mut self, event: &E) {
        self(event)
    }
}

pub trait MaybeHandleEvent<E: ?Sized>: Into<Option<Self::HandleEvent>> {
    type HandleEvent: HandleEvent<E>;
}

// TODO: the compiler doesn't allow relaxing the bound to `F: HandleEvent<E>`
impl<F: FnMut(&E), E: ?Sized> MaybeHandleEvent<E> for F {
    type HandleEvent = F;
}

impl<F: HandleEvent<E>, E: ?Sized> MaybeHandleEvent<E> for Option<F> {
    type HandleEvent = F;
}

pub struct HandleEventWithOptions<H>(pub H, pub EventListenerOptions);

impl<H: HandleEvent<E>, E: ?Sized> HandleEvent<E> for HandleEventWithOptions<H> {
    fn handle_event(&mut self, event: &E) {
        self.0.handle_event(event)
    }

    fn event_listener_options(&self) -> EventListenerOptions {
        self.1
    }
}

impl<H: HandleEvent<E>, E: ?Sized> MaybeHandleEvent<E> for HandleEventWithOptions<H> {
    type HandleEvent = Self;
}
