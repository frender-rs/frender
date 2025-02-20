use frender_common::Empty;

use crate::known_str::KnownIsNonReactiveStr;

use super::InputType;

// TODO: export?
#[derive(PartialEq, Eq)]
pub enum NeverStr {}

impl AsRef<str> for NeverStr {
    fn as_ref(&self) -> &str {
        match *self {}
    }
}

impl InputType for Empty {
    type InputTypeStr = NeverStr;

    fn maybe_into_input_type_str(Self: Self) -> Option<Self::InputTypeStr> {
        None
    }
}

impl<S: KnownIsNonReactiveStr> InputType for S {
    type InputTypeStr = S;

    fn maybe_into_input_type_str(this: Self) -> Option<Self::InputTypeStr> {
        Some(this)
    }
}

impl<S: InputType> InputType for Option<S> {
    type InputTypeStr = S::InputTypeStr;

    fn maybe_into_input_type_str(this: Self) -> Option<Self::InputTypeStr> {
        this.and_then(S::maybe_into_input_type_str)
    }
}

// TODO: Either
// #[cfg(feature = "either")]
// impl<L: InputType, R: InputType> InputType for either::Either<L, R> {
//     type InputTypeStr;

//     fn maybe_into_input_type_str(this: Self) -> Option<Self::InputTypeStr> {
//         todo!()
//     }
// }
