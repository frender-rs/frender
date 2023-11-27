use std::pin::Pin;

use crate::{AsyncStrIterator, IntoAsyncStrIterator};

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

#[allow(non_snake_case)]
impl<L, R> IterEither<L, R> {
    pub fn Left(left: L) -> Self {
        Self::Left { inner: left }
    }
    pub fn Right(right: R) -> Self {
        Self::Right { inner: right }
    }
}

#[cfg(feature = "either")]
mod imp {
    use either::Either;

    use crate::IntoAsyncStrIterator;

    use super::IterEither;

    impl<L: IntoAsyncStrIterator, R: IntoAsyncStrIterator> IntoAsyncStrIterator for Either<L, R> {
        type IntoAsyncStrIterator = IterEither<L::IntoAsyncStrIterator, R::IntoAsyncStrIterator>;

        fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
            match self {
                Either::Left(this) => IterEither::Left {
                    inner: this.into_async_str_iterator(),
                },
                Either::Right(this) => IterEither::Right {
                    inner: this.into_async_str_iterator(),
                },
            }
        }
    }
}
