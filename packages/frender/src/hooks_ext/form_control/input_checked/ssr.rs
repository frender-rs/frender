use frender_html::form_control::{input::SsrInputChecked, KindOfChecked};
use hooks::Signal;

use crate::hooks_ext::form_control::ToProvideFormControlValue;

use super::super::SignalIntoControlledValue;

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
