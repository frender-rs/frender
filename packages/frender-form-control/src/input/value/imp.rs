use frender_dom::Empty;

use crate::{
    known::KnownStr,
    value::{KindOfValue, KindOfValueAsNumber},
    values::UncontrolledWithDefaultValue,
};

use super::InputValue;

// Uncontrolled input.defaultValue that is always an empty string.
impl InputValue for Empty {
    type ValueKind = KindOfValue;
}

// Uncontrolled number or date/time with default value
impl InputValue for f64 {
    type ValueKind = KindOfValueAsNumber;
}
impl InputValue for UncontrolledWithDefaultValue<f64> {
    type ValueKind = KindOfValueAsNumber;
}

// Uncontrolled input.defaultValue that is a string
impl<T: KnownStr> InputValue for T {
    type ValueKind = KindOfValue;
}
impl<T: KnownStr> InputValue for UncontrolledWithDefaultValue<T> {
    type ValueKind = KindOfValue;
}

#[cfg(feature = "either")]
impl<L: InputValue, R: InputValue<ValueKind = L::ValueKind>> InputValue for either::Either<L, R> {
    type ValueKind = L::ValueKind;
}
