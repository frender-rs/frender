use crate::{RenderContext, RenderState};

#[derive(Default, Clone, Copy, Debug)]
pub struct NonReactiveRenderState<T>(pub T);

impl<T> Unpin for NonReactiveRenderState<T> {}

impl<T, Ctx: for<'ctx> RenderContext<'ctx>> RenderState<Ctx> for NonReactiveRenderState<T> {
    fn unmount(self: std::pin::Pin<&mut Self>) {}

    fn poll_reactive(
        self: std::pin::Pin<&mut Self>,
        _: &mut <Ctx as RenderContext<'_>>::ContextData,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        std::task::Poll::Ready(())
    }
}
