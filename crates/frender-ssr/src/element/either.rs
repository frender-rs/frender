use std::pin::Pin;

use either::Either;

use crate::{AsyncStrIterator, Element, RenderState};

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

impl<L: AsyncStrIterator, R: AsyncStrIterator> AsyncStrIterator for Either<L, R> {
    fn poll_next_str(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        match self.as_pin_mut() {
            Either::Left(s) => s.poll_next_str(cx),
            Either::Right(s) => s.poll_next_str(cx),
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

    type IntoIterStrings = Either<L::IntoIterStrings, R::IntoIterStrings>;

    fn into_iter_strings(self) -> Self::IntoIterStrings {
        match self {
            Either::Left(e) => Either::Left(e.into_iter_strings()),
            Either::Right(e) => Either::Right(e.into_iter_strings()),
        }
    }
}
