use either::Either;

use async_str_iter::either::IterEither;

use crate::Element;

impl<L, R> Element for Either<L, R>
where
    L: Element,
    R: Element,
{
    type HtmlChildren = IterEither<L::HtmlChildren, R::HtmlChildren>;

    fn into_html_children(self) -> Self::HtmlChildren {
        match self {
            Either::Left(e) => IterEither::Left(e.into_html_children()),
            Either::Right(e) => IterEither::Right(e.into_html_children()),
        }
    }
}
