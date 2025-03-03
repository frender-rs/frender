use frender_dom::Empty;

use crate::{
    csr::value::FormControlValue,
    known::KnownCsrStr,
    values::{UncontrolledEmptyDefaultValue, UncontrolledWithDefaultValue},
};

use super::InputValue;

pub trait CsrInputValue: InputValue {
    type IntoCsrInputValue: FormControlValue<Self::ValueKind>;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue;
}

impl CsrInputValue for Empty {
    type IntoCsrInputValue = UncontrolledEmptyDefaultValue;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        UncontrolledEmptyDefaultValue
    }
}

impl CsrInputValue for f64 {
    type IntoCsrInputValue = UncontrolledWithDefaultValue<Self>;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        UncontrolledWithDefaultValue(self)
    }
}
impl CsrInputValue for UncontrolledWithDefaultValue<f64> {
    type IntoCsrInputValue = Self;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        self
    }
}

impl<T: KnownCsrStr> CsrInputValue for T {
    type IntoCsrInputValue = UncontrolledWithDefaultValue<T>;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        UncontrolledWithDefaultValue(self)
    }
}
impl<T: KnownCsrStr> CsrInputValue for UncontrolledWithDefaultValue<T> {
    type IntoCsrInputValue = Self;

    fn into_csr_input_value(self) -> Self::IntoCsrInputValue {
        self
    }
}
