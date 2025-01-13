pub use frender_common::{HandleEvent, MaybeHandleEvent};

pub trait RegisterOrUpdate<N: ?Sized, R: ?Sized, F> {
    fn register_or_update(self: std::pin::Pin<&mut Self>, node: &mut N, renderer: &mut R, f: F);
}

pub trait RegisterUpdate<N: ?Sized, R: ?Sized, F> {
    fn register(node: &mut N, renderer: &mut R, f: F) -> Self;
    fn update(&mut self, node: &mut N, renderer: &mut R, f: F);
}
