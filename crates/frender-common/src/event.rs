/// This trait exists because we can't impl [`FnMut`] on stable rust.
pub trait HandleEvent<E: ?Sized> {
    fn handle_event(&mut self, event: &E);
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
