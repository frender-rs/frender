use frender_html::form_control::input::{InputValue, InputValueKind};

use hooks::Signal;

use super::{SignalIntoControlledValue, ToProvideFormControlValueWithKind};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl<S, FK> InputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = FK>,
    FK: InputValueKind,
{
    type ValueKind = FK;
}
