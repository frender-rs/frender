use std::pin::Pin;

use either::Either;

use crate::RenderState;

impl<Renderer, L: RenderState<Renderer>, R: RenderState<Renderer>> RenderState<Renderer>
    for Either<L, R>
{
    fn unmount(self: Pin<&mut Self>, renderer: &mut Renderer) {
        match self.as_pin_mut() {
            Either::Left(s) => s.unmount(renderer),
            Either::Right(s) => s.unmount(renderer),
        }
    }

    fn state_unmount(self: std::pin::Pin<&mut Self>) {
        match self.as_pin_mut() {
            Either::Left(s) => s.state_unmount(),
            Either::Right(s) => s.state_unmount(),
        }
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut Renderer,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_render(renderer, cx),
            Either::Right(s) => s.poll_render(renderer, cx),
        }
    }
}

pin_project_lite::pin_project!(
    pub struct EitherState<A, B> {
        #[pin]
        inner: Either<A, B>,
    }
);

impl<A: Default, B: Default> Default for EitherState<A, B> {
    fn default() -> Self {
        EitherState {
            inner: Either::Left(A::default()),
        }
    }
}

impl<R, A: RenderState<R>, B: RenderState<R>> RenderState<R> for EitherState<A, B> {
    fn unmount(self: Pin<&mut Self>, renderer: &mut R) {
        self.project().inner.unmount(renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project().inner.state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project().inner.poll_render(renderer, cx)
    }
}
