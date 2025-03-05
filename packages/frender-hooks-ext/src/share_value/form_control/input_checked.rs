use frender_form_control::{KindOfChecked, input::InputChecked};
use hooks::Signal;

use super::{SignalIntoControlledValue, ToProvideFormControlValue};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl<S> InputChecked for SignalIntoControlledValue<S>
where
    S: Signal + 'static,
    S::Value: ToProvideFormControlValue<KindOfChecked>,
{
}
