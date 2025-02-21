use frender_html::form_control::{input::InputChecked, KindOfChecked, ProvideFormControlValue};
use hooks::Signal;

use super::SignalIntoControlledValue;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl<S> InputChecked for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ProvideFormControlValue<KindOfChecked>,
{
}
