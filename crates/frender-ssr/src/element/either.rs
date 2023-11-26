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

pin_project_lite::pin_project!(
    #[project = IterEitherProj]
    pub enum IterEither<L, R> {
        Left {
            #[pin]
            inner: L,
        },
        Right {
            #[pin]
            inner: R,
        },
    }
);

impl<L: AsyncStrIterator, R: AsyncStrIterator> AsyncStrIterator for IterEither<L, R> {
    fn poll_next_str(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        match self.project() {
            IterEitherProj::Left { inner } => inner.poll_next_str(cx),
            IterEitherProj::Right { inner } => inner.poll_next_str(cx),
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

    type IntoAsyncHtmlChunks = IterEither<L::IntoAsyncHtmlChunks, R::IntoAsyncHtmlChunks>;

    fn into_async_html_chunks(self) -> Self::IntoAsyncHtmlChunks {
        match self {
            Either::Left(e) => IterEither::Left {
                inner: e.into_async_html_chunks(),
            },
            Either::Right(e) => IterEither::Right {
                inner: e.into_async_html_chunks(),
            },
        }
    }
}
