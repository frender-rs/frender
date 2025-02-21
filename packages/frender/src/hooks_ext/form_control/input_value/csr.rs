use frender_html::form_control::{
    input::{CsrInputValue, InputValueKind},
    ProvideFormControlValueWithKind,
};
use hooks::Signal;

use crate::hooks_ext::form_control::{FromFormControlValue, SignalIntoControlledValue};

impl<S, FK> CsrInputValue for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ProvideFormControlValueWithKind<ProvideFormControlValueKind = FK>,
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
