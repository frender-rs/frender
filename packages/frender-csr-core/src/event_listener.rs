use std::pin::Pin;

pub use frender_common::{HandleEvent, MaybeHandleEvent};
use frender_reactive_value::RenderInitPinned;

pub trait PinnedRegisterUpdate<N: ?Sized, R: ?Sized, F> {
    type PinnedRegisterInit: for<'n, 'r> RenderInitPinned<
        //
        (&'n mut N, &'r mut R),
        Self,
        Output = (),
    >;
    fn pinned_register_init(
        node: &mut N,
        renderer: &mut R,
        f: F,
    ) -> (Self, Self::PinnedRegisterInit)
    where
        Self: Sized;
    fn pinned_update(self: Pin<&mut Self>, node: &mut N, renderer: &mut R, f: F);
}

pub trait RegisterUpdate<N: ?Sized, R: ?Sized, F> {
    fn register(node: &mut N, renderer: &mut R, f: F) -> Self;
    fn update(&mut self, node: &mut N, renderer: &mut R, f: F);
}

/*
impl<
        //
        T: RegisterUpdate<N, R, F> + Unpin,
        N: ?Sized,
        R: ?Sized,
        F,
    > PinnedRegisterUpdate<N, R, F> for T
{
    type PinnedRegisterInit = ();
    fn pinned_register(node: &mut N, renderer: &mut R, f: F) -> Self {
        T::register(node, renderer, f)
    }

    fn pinned_register_init(
        self: Pin<&mut Self>,
        _: &mut N,
        _: &mut R,
        (): Self::PinnedRegisterInit,
    ) {
    }

    fn pinned_update(self: Pin<&mut Self>, node: &mut N, renderer: &mut R, f: F) {
        self.get_mut().update(node, renderer, f)
    }
}
*/
