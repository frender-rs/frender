use async_str_iter::either::IterEither;
use frender_common::either::EitherElement;

use crate::SsrElement;

impl<A, B> SsrElement for EitherElement<A, B>
where
    A: SsrElement,
    B: SsrElement,
{
    type HtmlChildren = IterEither<A::HtmlChildren, B::HtmlChildren>;

    fn into_html_children(self) -> Self::HtmlChildren {
        match self {
            EitherElement::A(e) => IterEither::Left(e.into_html_children()),
            EitherElement::B(e) => IterEither::Right(e.into_html_children()),
        }
    }
}

impl<A, B> super::KnownCopySsrElement for EitherElement<A, B>
where
    A: SsrElement + Copy,
    B: SsrElement + Copy,
{
}

#[cfg(feature = "either")]
mod extern_either {
    use either::Either;
    use frender_common::either::EitherElement;

    use crate::SsrElement;

    impl<L, R> SsrElement for Either<L, R>
    where
        L: SsrElement,
        R: SsrElement,
    {
        type HtmlChildren = <EitherElement<L, R> as SsrElement>::HtmlChildren;

        fn into_html_children(self) -> Self::HtmlChildren {
            match self {
                Either::Left(this) => EitherElement::A(this),
                Either::Right(this) => EitherElement::B(this),
            }
            .into_html_children()
        }
    }

    impl<A, B> super::super::KnownCopySsrElement for Either<A, B>
    where
        A: SsrElement + Copy,
        B: SsrElement + Copy,
    {
    }
}
