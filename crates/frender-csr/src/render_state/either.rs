use std::pin::Pin;

use either::Either;

use crate::RenderState;

impl<
        PEH: ?Sized,
        Renderer: ?Sized,
        L: RenderState<PEH, Renderer>,
        R: RenderState<PEH, Renderer>,
    > RenderState<PEH, Renderer> for Either<L, R>
{
    fn unmount(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut Renderer) {
        match self.as_pin_mut() {
            Either::Left(s) => s.unmount(peh, renderer),
            Either::Right(s) => s.unmount(peh, renderer),
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
        peh: &mut PEH,
        renderer: &mut Renderer,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_render(peh, renderer, cx),
            Either::Right(s) => s.poll_render(peh, renderer, cx),
        }
    }
}

pin_project_lite::pin_project!(
    pub struct EitherRenderState<A, B> {
        #[pin]
        inner: Either<A, B>,
    }
);

impl<A, B> EitherRenderState<A, B> {
    pub fn inner_mut(&mut self) -> &mut Either<A, B> {
        &mut self.inner
    }

    pub fn project_inner(self: Pin<&mut Self>) -> Pin<&mut Either<A, B>> {
        self.project().inner
    }
}

impl<A: Default, B: Default> Default for EitherRenderState<A, B> {
    fn default() -> Self {
        EitherRenderState {
            inner: Either::Left(A::default()),
        }
    }
}

impl<PEH: ?Sized, R: ?Sized, A: RenderState<PEH, R>, B: RenderState<PEH, R>> RenderState<PEH, R>
    for EitherRenderState<A, B>
{
    fn unmount(self: Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
        self.project().inner.unmount(peh, renderer)
    }

    fn state_unmount(self: Pin<&mut Self>) {
        self.project().inner.state_unmount()
    }

    fn poll_render(
        self: Pin<&mut Self>,
        peh: &mut PEH,
        renderer: &mut R,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<()> {
        self.project().inner.poll_render(peh, renderer, cx)
    }
}
