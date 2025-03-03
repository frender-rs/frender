use std::borrow::Borrow;

use frender_reactive_value::{
    into_borrow_str::IntoBorrowStr,
    non_reactive::{Uncached, UncachedNonReactiveValue},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
    ProvideValueOfKind,
};

use crate::{
    known::{KnownIntoBorrowStr, KnownStr},
    KindOfValue, MaybeProvideFormControlValue, ProvideFormControlValue,
};

pub trait IntoMaybeProvideStr {
    type IntoMaybeProvideStr: MaybeProvideFormControlValue<KindOfValue>;
    fn into_maybe_provide_str(self) -> Self::IntoMaybeProvideStr;
}

pub(crate) trait KnownProvideStr: IntoMaybeProvideStr + KnownStr {}

impl<T: KnownIntoBorrowStr> KnownProvideStr for T {}
impl<T: KnownIntoBorrowStr> IntoMaybeProvideStr for T {
    type IntoMaybeProvideStr = ProvideIntoBorrowStr<T::IntoBorrowStr>;

    fn into_maybe_provide_str(self) -> Self::IntoMaybeProvideStr {
        ProvideIntoBorrowStr(self.into_borrow_str())
    }
}

impl<T: UncachedNonReactiveValue<KindOfTempRef<str>>> KnownProvideStr for Uncached<T> {}
impl<T: UncachedNonReactiveValue<KindOfTempRef<str>>> IntoMaybeProvideStr for Uncached<T> {
    type IntoMaybeProvideStr = ProvideProvideTempRefStr<T::UncachedIntoProvideValue>;

    fn into_maybe_provide_str(self) -> Self::IntoMaybeProvideStr {
        ProvideProvideTempRefStr(self.0.uncached_into_provide_value())
    }
}

pub struct ProvideIntoBorrowStr<T: IntoBorrowStr>(T);

impl<T: Borrow<str>> MaybeProvideFormControlValue<KindOfValue> for ProvideIntoBorrowStr<T> {
    type ProvideFormControlValue = Self;
    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        Some(this)
    }
}

impl<T: Borrow<str>> ProvideFormControlValue<KindOfValue> for ProvideIntoBorrowStr<T> {
    fn provide_form_control_value<R>(
        self,
        receive: impl FnOnce(<KindOfValue as crate::FormControlValueKind>::Value<'_>) -> R,
    ) -> R {
        receive(self.0.borrow())
    }
}

pub struct ProvideProvideTempRefStr<T: ProvideValueOfKind<KindOfTempRef<str>>>(T);

impl<T: ProvideValueOfKind<KindOfTempRef<str>>> MaybeProvideFormControlValue<KindOfValue>
    for ProvideProvideTempRefStr<T>
{
    type ProvideFormControlValue = Self;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        Some(this)
    }
}

impl<T: ProvideValueOfKind<KindOfTempRef<str>>> ProvideFormControlValue<KindOfValue>
    for ProvideProvideTempRefStr<T>
{
    fn provide_form_control_value<R>(
        self,
        receive: impl FnOnce(<KindOfValue as crate::FormControlValueKind>::Value<'_>) -> R,
    ) -> R {
        self.0.provide_value_of_kind(|TempRef(v)| receive(v))
    }
}
