use frender_html::form_control::{
    input::{InputValue, InputValueKind},
    ProvideFormControlValueWithKind,
};

use hooks::Signal;

use super::SignalIntoControlledValue;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl<S, FK> InputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ProvideFormControlValueWithKind<ProvideFormControlValueKind = FK>,
    FK: InputValueKind,
{
    type ValueKind = FK;
}
