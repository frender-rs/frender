/// This trait exists because we can't impl [`FnMut`] on stable rust.
pub trait HandleEvent<E: ?Sized> {
    fn handle_event(&mut self, event: &E);
}

impl<E: ?Sized, F: FnMut(&E)> HandleEvent<E> for F {
    fn handle_event(&mut self, event: &E) {
        self(event)
    }
}

pub trait EventListenerState<N: ?Sized, R: ?Sized, F>: Default + RegisterOrUpdate<N, R, F> {
    type EventListenerStateUnpinned: Default + Unpin + RegisterOrUpdate<N, R, F>;
}

pub trait RegisterOrUpdate<N: ?Sized, R: ?Sized, F> {
    fn register_or_update(
        self: std::pin::Pin<&mut Self>,
        node: &mut N,
        renderer: &mut R,
        event_type: impl Into<std::borrow::Cow<'static, str>>,
        f: F,
    );
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
