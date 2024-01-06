use std::{pin::Pin, task::Poll};

pub trait RenderState<PEH: ?Sized, R: ?Sized> {
    fn unmount(self: Pin<&mut Self>, parent_elements_handle: &mut PEH, renderer: &mut R);

    fn state_unmount(self: Pin<&mut Self>);

    fn poll_render(
        self: Pin<&mut Self>,
        parent_elements_handle: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<()>;
}
