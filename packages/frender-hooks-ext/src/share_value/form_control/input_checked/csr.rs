use frender_form_control::{input::CsrInputChecked, KindOfChecked};
use hooks::Signal;

use super::super::{FromFormControlValue, SignalIntoControlledValue, ToProvideFormControlValue};

impl<S> CsrInputChecked for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValue<KindOfChecked>,
    S::SignalHook: Unpin,
    S::Value: FromFormControlValue<KindOfChecked>,
{
    type IntoCsrInputChecked = Self;

    fn into_csr_input_checked(self) -> Self::IntoCsrInputChecked {
        self
    }
}
