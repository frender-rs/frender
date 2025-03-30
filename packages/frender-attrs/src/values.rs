pub use frender_common::Empty;

use crate::IntoAttributes;

mod empty;
impl IntoAttributes for Empty {
    type IntoAttributes = empty::EmptyAttributes;

    fn into_attributes(self) -> Self::IntoAttributes {
        empty::EmptyAttributes
    }
}

mod option;
impl<T: IntoAttributes> IntoAttributes for Option<T> {
    type IntoAttributes = option::OptionAttributes<T::IntoAttributes>;

    fn into_attributes(self) -> Self::IntoAttributes {
        option::OptionAttributes(self.map(T::into_attributes))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EitherAttributes<A, B> {
    A(A),
    B(B),
}
mod either;
impl<A: IntoAttributes, B: IntoAttributes> IntoAttributes for EitherAttributes<A, B> {
    type IntoAttributes = EitherAttributes<A::IntoAttributes, B::IntoAttributes>;

    fn into_attributes(self) -> Self::IntoAttributes {
        match self {
            EitherAttributes::A(this) => EitherAttributes::A(this.into_attributes()),
            EitherAttributes::B(this) => EitherAttributes::B(this.into_attributes()),
        }
    }
}

#[cfg(feature = "either")]
impl<A: IntoAttributes, B: IntoAttributes> IntoAttributes for ::either::Either<A, B> {
    type IntoAttributes = EitherAttributes<A::IntoAttributes, B::IntoAttributes>;

    fn into_attributes(self) -> Self::IntoAttributes {
        match self {
            Self::Left(this) => EitherAttributes::A(this.into_attributes()),
            Self::Right(this) => EitherAttributes::B(this.into_attributes()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);
mod chain;

pub mod r#const;
