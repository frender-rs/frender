use frender_html::form_control::{input::SsrInputChecked, KindOfChecked, ProvideFormControlValue};
use hooks::Signal;

use super::super::SignalIntoControlledValue;

impl<S> SsrInputChecked for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ProvideFormControlValue<KindOfChecked>,
{
    type IntoSsrInputChecked = Self;

    fn into_ssr_input_checked(self) -> Self::IntoSsrInputChecked {
        self
    }
}
