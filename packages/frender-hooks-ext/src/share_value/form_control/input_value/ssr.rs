use frender_form_control::input::{InputValueKind, SsrInputValue};
use hooks::Signal;

use super::super::{SignalIntoControlledValue, ToProvideFormControlValueWithKind};

impl<S, FK> SsrInputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = FK>,
    FK: InputValueKind,
{
    type IntoSsrInputValue = Self;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        self
    }
}
