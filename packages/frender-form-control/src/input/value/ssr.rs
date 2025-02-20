use crate::ssr::value::MaybeProvideFormControlValue;

use super::InputValue;

pub trait SsrInputValue: InputValue {
    type IntoSsrInputValue: MaybeProvideFormControlValue<Self::ValueKind>;
    fn into_ssr_input_value(self) -> Self::IntoSsrInputValue;
}
