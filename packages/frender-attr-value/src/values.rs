use crate::{AttrValueKind, IntoAttrValue};

#[derive(Debug, Clone, Copy)]
pub enum Never {}
mod never;
impl<VK: AttrValueKind> IntoAttrValue<VK> for Never {
    type IntoAttrValue = never::NeverAttrValue;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        match self {}
    }
}

mod option;
impl<T: IntoAttrValue<VK>, VK: AttrValueKind> IntoAttrValue<VK> for Option<T> {
    type IntoAttrValue = option::OptionAttrValue<T::IntoAttrValue>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        option::OptionAttrValue(self.map(T::into_attr_value))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EitherAttrValue<A, B> {
    A(A),
    B(B),
}
mod either;
impl<A: IntoAttrValue<VK>, B: IntoAttrValue<VK>, VK: AttrValueKind> IntoAttrValue<VK>
    for EitherAttrValue<A, B>
{
    type IntoAttrValue = EitherAttrValue<A::IntoAttrValue, B::IntoAttrValue>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        match self {
            EitherAttrValue::A(this) => EitherAttrValue::A(this.into_attr_value()),
            EitherAttrValue::B(this) => EitherAttrValue::B(this.into_attr_value()),
        }
    }
}
#[cfg(feature = "either")]
impl<A: IntoAttrValue<VK>, B: IntoAttrValue<VK>, VK: AttrValueKind> IntoAttrValue<VK>
    for ::either::Either<A, B>
{
    type IntoAttrValue = EitherAttrValue<A::IntoAttrValue, B::IntoAttrValue>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        match self {
            Self::Left(this) => EitherAttrValue::A(this.into_attr_value()),
            Self::Right(this) => EitherAttrValue::B(this.into_attr_value()),
        }
    }
}

/// Indicates an attribute is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Absent;
mod absent;
impl<VK: AttrValueKind> IntoAttrValue<VK> for Absent {
    type IntoAttrValue = absent::AbsentAttributeValue;
    fn into_attr_value(self) -> Self::IntoAttrValue {
        absent::AbsentAttributeValue
    }
}

pub mod r#const;

pub(crate) mod cached_some;
