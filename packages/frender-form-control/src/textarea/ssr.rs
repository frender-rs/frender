use async_str_iter::IntoAsyncStrIterator;
use frender_ssr::html::assert::SafeTextOrEmpty;
use frender_ssr::html::encode::Encode;
use frender_ssr::html::escape_safe::Safe;

use frender_common::Empty;
use frender_reactive_value::ssr::SsrStr;

use crate::known::KnownSsrStr;
use crate::values::{EitherFormControlValue, UncontrolledWithDefaultValue};

use super::TextAreaValue;

pub trait SsrTextAreaValue: TextAreaValue {
    type IntoSsrTextAreaValue: SafeTextOrEmpty;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue;
}

impl SsrTextAreaValue for Empty {
    type IntoSsrTextAreaValue = async_str_iter::empty::Empty;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        async_str_iter::empty::Empty
    }
}

impl<S: KnownSsrStr> SsrTextAreaValue for S {
    type IntoSsrTextAreaValue =
        <UncontrolledWithDefaultValue<S> as SsrTextAreaValue>::IntoSsrTextAreaValue;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        UncontrolledWithDefaultValue(self).into_ssr_text_area_value()
    }
}

impl<S: SsrStr> SsrTextAreaValue for UncontrolledWithDefaultValue<S> {
    type IntoSsrTextAreaValue = Encode<Safe, S::SsrStrIntoAsyncStrIterator>;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        let Self(this) = self;
        Encode::new(Safe, this.ssr_str_into_async_str_iterator())
    }
}

impl<T: SsrTextAreaValue> SsrTextAreaValue for Option<T> {
    type IntoSsrTextAreaValue = async_str_iter::option::IterOption<T::IntoSsrTextAreaValue>;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        self.map(T::into_ssr_text_area_value)
            .into_async_str_iterator()
    }
}

impl<A: SsrTextAreaValue, B: SsrTextAreaValue> SsrTextAreaValue for EitherFormControlValue<A, B> {
    type IntoSsrTextAreaValue =
        async_str_iter::either::IterEither<A::IntoSsrTextAreaValue, B::IntoSsrTextAreaValue>;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        use async_str_iter::either::IterEither;
        match self {
            EitherFormControlValue::A(this) => IterEither::Left(this.into_ssr_text_area_value()),
            EitherFormControlValue::B(this) => IterEither::Right(this.into_ssr_text_area_value()),
        }
    }
}

#[cfg(feature = "either")]
impl<L: SsrTextAreaValue, R: SsrTextAreaValue> SsrTextAreaValue for either::Either<L, R> {
    type IntoSsrTextAreaValue =
        async_str_iter::either::IterEither<L::IntoSsrTextAreaValue, R::IntoSsrTextAreaValue>;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        match self {
            Self::Left(this) => EitherFormControlValue::A(this),
            Self::Right(this) => EitherFormControlValue::B(this),
        }
        .into_ssr_text_area_value()
    }
}
