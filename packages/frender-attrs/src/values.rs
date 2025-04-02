pub use frender_common::Empty;

use crate::IntoAttributes;

mod empty;
impl IntoAttributes for Empty {
    type IntoAttributes = empty::EmptyAttributes;

    fn into_attributes(self) -> Self::IntoAttributes {
        empty::EmptyAttributes
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Never {}
mod never;
impl IntoAttributes for Never {
    type IntoAttributes = never::NeverAttributes;

    fn into_attributes(self) -> Self::IntoAttributes {
        match self {}
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
impl<A: IntoAttributes, B: IntoAttributes> IntoAttributes for Chain<A, B> {
    type IntoAttributes = chain::ChainAttributes<A::IntoAttributes, B::IntoAttributes>;

    fn into_attributes(self) -> Self::IntoAttributes {
        let Self(a, b) = self;
        chain::ChainAttributes(a.into_attributes(), b.into_attributes())
    }
}

pub mod r#const;
impl<M: ?Sized + r#const::HasConstAttributes> IntoAttributes for r#const::ConstAttributes<M> {
    type IntoAttributes = Self;

    fn into_attributes(self) -> Self::IntoAttributes {
        self
    }
}
