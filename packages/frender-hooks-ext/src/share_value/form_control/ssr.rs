use frender_form_control::{
    FormControlValueKind, MaybeProvideFormControlValue, ProvideFormControlValue,
};
use hooks::ShareValue;

use super::{SignalIntoControlledValue, ToProvideFormControlValue};

impl<S, Val, VK> ProvideFormControlValue<VK> for SignalIntoControlledValue<S>
where
    S: ShareValue<Value = Val>,
    Val: ToProvideFormControlValue<VK>,
    VK: FormControlValueKind,
{
    fn provide_form_control_value<R>(self, receive: impl FnOnce(VK::Value<'_>) -> R) -> R {
        self.0
            .map(|value| value.to_provide_form_control_value(receive))
    }
}

impl<S, Val, VK> MaybeProvideFormControlValue<VK> for SignalIntoControlledValue<S>
where
    S: ShareValue<Value = Val>,
    Val: ToProvideFormControlValue<VK>,
    VK: FormControlValueKind,
{
    type ProvideFormControlValue = Self;

    fn maybe_into_provide_form_control_value(this: Self) -> Option<Self::ProvideFormControlValue> {
        Some(this)
    }
}
