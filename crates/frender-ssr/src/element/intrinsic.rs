#![cfg(prototyping)] // TODO: remove

use frender_core::RenderState;

use crate::{
    async_writable::{AsyncWritable, Chain},
    bytes::CowSlicedBytes,
};

pin_project_lite::pin_project!(
    pub struct IntrinsicElementState<ChildrenState, Attrs: AsyncWritable> {
        #[pin]
        inner: Chain![
            &'static [u8], // <
            &'static [u8], // div
            ChildrenState
        ],
        #[pin]
        before_children: Attrs,
        #[pin]
        children: ChildrenState,
        after_children: Attrs,
    }
);

impl<ChildrenState: RenderState, Props: AsyncWritable> RenderState
    for IntrinsicElementState<ChildrenState, Props>
{
    fn unmount(self: std::pin::Pin<&mut Self>) {
        self.project().children.unmount()
    }

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        let _ = cx;
        std::task::Poll::Ready(())
    }
}
