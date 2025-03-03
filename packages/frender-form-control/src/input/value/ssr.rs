use frender_dom::Empty;
use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

use crate::{
    known::KnownStaticOrIntoStaticStr, known_provide_str::KnownProvideStr,
    values::UncontrolledWithDefaultValue, KindOfValue, MaybeProvideFormControlValue,
    ProvideFormControlValue,
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

impl<T: KnownProvideStr> SsrInputValue for T {
    type IntoSsrInputValue = T::IntoMaybeProvideStr;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        T::into_maybe_provide_str(self)
    }
}
impl<T: KnownProvideStr> SsrInputValue for UncontrolledWithDefaultValue<T> {
    type IntoSsrInputValue = T::IntoMaybeProvideStr;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        T::into_maybe_provide_str(self.0)
    }
}
