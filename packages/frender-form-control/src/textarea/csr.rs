use frender_common::reactive_value::{
    non_reactive::{Uncached, UncachedNonReactiveValue},
    ReactiveValue,
};
use frender_dom::Empty;

use crate::{
    csr::value::FormControlValue,
    known_str::{KnownCsrStr, KnownReactiveValueStr},
    value::KindOfValue,
    values::{EitherFormControlValue, UncontrolledEmptyDefaultValue, UncontrolledWithDefaultValue},
};

use super::TextAreaValue;

pub trait CsrTextAreaValue: TextAreaValue {
    type IntoCsrTextAreaValue: FormControlValue<KindOfValue>;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue;
}

impl CsrTextAreaValue for Empty {
    type IntoCsrTextAreaValue = UncontrolledEmptyDefaultValue;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        UncontrolledEmptyDefaultValue
    }
}

impl<T: KnownReactiveValueStr> CsrTextAreaValue for T {
    type IntoCsrTextAreaValue = UncontrolledWithDefaultValue<Self>;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        UncontrolledWithDefaultValue(self)
    }
}

impl<T: ReactiveValue<str>> CsrTextAreaValue for UncontrolledWithDefaultValue<T> {
    type IntoCsrTextAreaValue = Self;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        self
    }
}

impl<T: CsrTextAreaValue> CsrTextAreaValue for Option<T> {
    type IntoCsrTextAreaValue = Option<T::IntoCsrTextAreaValue>;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        self.map(T::into_csr_text_area_value)
    }
}

impl<A: CsrTextAreaValue, B: CsrTextAreaValue> CsrTextAreaValue for EitherFormControlValue<A, B> {
    type IntoCsrTextAreaValue =
        EitherFormControlValue<A::IntoCsrTextAreaValue, B::IntoCsrTextAreaValue>;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        match self {
            EitherFormControlValue::A(this) => {
                EitherFormControlValue::A(A::into_csr_text_area_value(this))
            }
            EitherFormControlValue::B(this) => {
                EitherFormControlValue::B(B::into_csr_text_area_value(this))
            }
        }
    }
}

#[cfg(feature = "either")]
impl<A: CsrTextAreaValue, B: CsrTextAreaValue> CsrTextAreaValue for either::Either<A, B> {
    type IntoCsrTextAreaValue =
        EitherFormControlValue<A::IntoCsrTextAreaValue, B::IntoCsrTextAreaValue>;

    fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
        match self {
            either::Either::Left(this) => {
                EitherFormControlValue::A(A::into_csr_text_area_value(this))
            }
            either::Either::Right(this) => {
                EitherFormControlValue::B(B::into_csr_text_area_value(this))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use frender_common::{
        reactive_value::{non_reactive::Uncached, ReactiveValue},
        TempStr,
    };
    use frender_dom::Empty;

    use crate::values::{EitherFormControlValue, UncontrolledWithDefaultValue};

    use super::CsrTextAreaValue;

    struct Test
    where
        Empty: CsrTextAreaValue,
        &'static str: CsrTextAreaValue,
        for<'a> Uncached<&'a str>: CsrTextAreaValue,
        for<'a> Uncached<TempStr<&'a str>>: CsrTextAreaValue,
        for<'a> Uncached<TempStr<std::borrow::Cow<'a, str>>>: CsrTextAreaValue,
        String: CsrTextAreaValue,
        std::borrow::Cow<'static, str>: CsrTextAreaValue,
        std::rc::Rc<str>: CsrTextAreaValue,
        std::sync::Arc<str>: CsrTextAreaValue;

    const _: Test = Test;

    trait Tests {
        type UncontrolledWithDefaultValue<V: ReactiveValue<str>>: CsrTextAreaValue;

        type Option<T: CsrTextAreaValue>: CsrTextAreaValue;

        type EitherFormControlValue<A: CsrTextAreaValue, B: CsrTextAreaValue>: CsrTextAreaValue;

        #[cfg(feature = "either")]
        type Either<L: CsrTextAreaValue, R: CsrTextAreaValue>: CsrTextAreaValue;
    }

    struct TestsImpl;

    impl Tests for TestsImpl {
        type UncontrolledWithDefaultValue<V: ReactiveValue<str>> = UncontrolledWithDefaultValue<V>;

        type Option<T: CsrTextAreaValue> = Option<T>;

        type EitherFormControlValue<A: CsrTextAreaValue, B: CsrTextAreaValue> =
            EitherFormControlValue<A, B>;

        #[cfg(feature = "either")]
        type Either<L: CsrTextAreaValue, R: CsrTextAreaValue> = either::Either<L, R>;
    }
}
