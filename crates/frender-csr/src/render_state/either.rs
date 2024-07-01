use std::pin::Pin;

use either::Either;

use crate::RenderState;

impl<Renderer: ?Sized, L: RenderState<Renderer>, R: RenderState<Renderer>> RenderState<Renderer>
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

    fn check_and_move_cursor(&self, render_context: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
        match self {
            Either::Left(this) => this.check_and_move_cursor(render_context),
            Either::Right(this) => this.check_and_move_cursor(render_context),
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

impl<R: ?Sized, A: RenderState<R>, B: RenderState<R>> RenderState<R> for EitherRenderState<A, B> {
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

    fn check_and_move_cursor(&self, render_context: &mut <R>::RenderContext<'_>)
    where
        R: crate::render::RenderWithContext,
    {
        self.inner.check_and_move_cursor(render_context)
    }
}
