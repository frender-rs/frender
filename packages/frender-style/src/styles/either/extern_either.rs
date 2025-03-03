use either::Either;

use crate::{IntoStyle, Style};

use super::EitherStyle;

impl<L: Style, R: Style> IntoStyle for Either<L, R> {
    type IntoStyle = EitherStyle<L, R>;

    fn into_style(self) -> Self::IntoStyle {
        match self {
            Either::Left(this) => EitherStyle::A(this),
            Either::Right(this) => EitherStyle::B(this),
        }
    }
}
