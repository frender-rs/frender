use frender_common::{reactive_value::non_reactive::Uncached, ToAsRefStr};
use frender_dom::Empty;

use crate::{
    known_str::KnownIsNonReactiveStr, values::UncontrolledWithDefaultValue, KindOfValue,
    MaybeProvideFormControlValue, ProvideFormControlValue,
};

use super::InputValue;

pub trait SsrInputValue: InputValue {
    type IntoSsrInputValue: MaybeProvideFormControlValue<Self::ValueKind>;
    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue;
}

impl SsrInputValue for Empty {
    type IntoSsrInputValue = Self;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        self
    }
}

impl SsrInputValue for f64 {
    type IntoSsrInputValue = Self;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        self
    }
}
impl SsrInputValue for UncontrolledWithDefaultValue<f64> {
    type IntoSsrInputValue = Self;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        self
    }
}

pub struct ProvideToAsRefStr<T: ToAsRefStr>(T);

impl<T: ToAsRefStr> MaybeProvideFormControlValue<KindOfValue> for ProvideToAsRefStr<T> {
    type ProvideFormControlValue = Self;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self> {
        Some(this)
    }
}

impl<T: ToAsRefStr> ProvideFormControlValue<KindOfValue> for ProvideToAsRefStr<T> {
    fn provide_form_control_value<R>(&self, receive: impl FnOnce(&str) -> R) -> R {
        receive(self.0.to_as_ref_str().as_ref())
    }
}

impl<T: KnownIsNonReactiveStr + ToAsRefStr> SsrInputValue for T {
    type IntoSsrInputValue = ProvideToAsRefStr<T>;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        ProvideToAsRefStr(self)
    }
}
impl<T: ToAsRefStr> SsrInputValue for Uncached<T> {
    type IntoSsrInputValue = ProvideToAsRefStr<T>;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        ProvideToAsRefStr(self.0)
    }
}
impl<T: KnownIsNonReactiveStr + ToAsRefStr> SsrInputValue for UncontrolledWithDefaultValue<T> {
    type IntoSsrInputValue = ProvideToAsRefStr<T>;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        ProvideToAsRefStr(self.0)
    }
}
impl<T: ToAsRefStr> SsrInputValue for UncontrolledWithDefaultValue<Uncached<T>> {
    type IntoSsrInputValue = ProvideToAsRefStr<T>;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        ProvideToAsRefStr(self.0 .0)
    }
}
