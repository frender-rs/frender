pub use frender_common::{HandleEvent, MaybeHandleEvent};

pub trait EventListenerState<N: ?Sized, R: ?Sized, F>: Default + RegisterOrUpdate<N, R, F> {
    type EventListenerStateUnpinned: Default + Unpin + RegisterOrUpdate<N, R, F>;
}

pub trait RegisterOrUpdate<N: ?Sized, R: ?Sized, F> {
    fn register_or_update(self: std::pin::Pin<&mut Self>, node: &mut N, renderer: &mut R, f: F);
}
