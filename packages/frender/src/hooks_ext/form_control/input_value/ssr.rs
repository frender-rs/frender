use frender_html::form_control::{
    input::{InputValueKind, SsrInputValue},
    ProvideFormControlValueWithKind,
};
use hooks::Signal;

use crate::hooks_ext::form_control::SignalIntoControlledValue;

impl<S, FK> SsrInputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ProvideFormControlValueWithKind<ProvideFormControlValueKind = FK>,
    FK: InputValueKind,
{
    type IntoSsrInputValue = Self;

    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue {
        self
    }
}
