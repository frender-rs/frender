use std::{pin::Pin, task::Poll};

pub trait RenderState<R: ?Sized> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    /// Implementation shouldn't change cursor
    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;

    // without updating the state
    fn check_and_move_cursor(&self, render_context: &mut R::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext;
}
