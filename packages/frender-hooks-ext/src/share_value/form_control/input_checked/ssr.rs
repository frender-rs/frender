use frender_form_control::{input::SsrInputChecked, KindOfChecked};
use hooks::Signal;

use super::super::{SignalIntoControlledValue, ToProvideFormControlValue};

impl<S> SsrInputChecked for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValue<KindOfChecked>,
{
    type IntoSsrInputChecked = Self;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        self
    }
}
