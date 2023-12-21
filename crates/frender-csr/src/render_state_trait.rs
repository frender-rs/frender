use std::{pin::Pin, task::Poll};

pub trait RenderState<R> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}
