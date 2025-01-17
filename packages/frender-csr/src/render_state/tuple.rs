use std::pin::Pin;

use crate::{RenderState, StateUnmount};

impl<R: ?Sized> RenderState<R> for () {
    #[inline]
    fn unmount(self: std::pin::Pin<&mut Self>, _: &mut R) {}

    #[inline]
    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    #[inline]
    fn poll_render(
        self: std::pin::Pin<&mut Self>,
        _: &mut R,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }

    fn check_and_move_cursor(&self, _: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
    }
}
