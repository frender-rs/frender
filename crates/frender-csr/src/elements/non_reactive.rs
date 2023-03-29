use crate::RenderState;

#[derive(Default, Clone, Copy, Debug)]
pub struct NonReactiveRenderState<T>(pub T);

impl<T> Unpin for NonReactiveRenderState<T> {}

impl<T> RenderState for NonReactiveRenderState<T> {
    fn unmount(self: std::pin::Pin<&mut Self>) {}

    fn poll_csr(
        self: std::pin::Pin<&mut Self>,
        _: &mut crate::CsrContext,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}
