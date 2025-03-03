use frender_html::form_control::input::{CsrInputValue, InputValueKind};
use hooks::Signal;

use crate::hooks_ext::form_control::{
    to_provide::ToProvideFormControlValueWithKind, FromFormControlValue, SignalIntoControlledValue,
};

impl<S, FK> CsrInputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValueWithKind<ToProvideFormControlValueKind = FK>,
    FK: InputValueKind,
    //
    S::SignalHook: Unpin,
    S::Value: FromFormControlValue<FK>,
{
    type IntoCsrInputValue = Self;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        self
    }
}
