use crate::RenderState;

#[derive(Default, Clone, Copy, Debug)]
pub struct NonReactiveRenderState<T>(pub T);

impl<T> Unpin for NonReactiveRenderState<T> {}

impl<T, R> RenderState<R> for NonReactiveRenderState<T> {
    fn unmount(self: std::pin::Pin<&mut Self>, renderer: &mut R) {}

    fn state_unmount(self: std::pin::Pin<&mut Self>) {}

    fn poll_render(self: std::pin::Pin<&mut Self>, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}
