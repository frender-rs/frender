use std::pin::Pin;

use either::Either;

use crate::{Element, RenderState};

impl<L: RenderState, R: RenderState> RenderState for Either<L, R> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: Pin<&mut Self>,
        writer: Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_render(writer, cx),
            Either::Right(s) => s.poll_render(writer, cx),
        }
    }
}

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type SsrState = Either<L::SsrState, R::SsrState>;

    fn into_ssr_state(self) -> Self::SsrState {
        match self {
            Either::Left(e) => Either::Left(e.into_ssr_state()),
            Either::Right(e) => Either::Right(e.into_ssr_state()),
        }
    }
}
