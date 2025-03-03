use std::borrow::Borrow;

use frender_common::Empty;

use crate::known::KnownStr;

use super::InputType;

// TODO: export?
#[derive(PartialEq, Eq)]
pub enum NeverStr {}

impl Borrow<str> for NeverStr {
    fn borrow(&self) -> &str {
        match *self {}
    }
}

impl InputType for Empty {
    type InputTypeStr = NeverStr;

    fn maybe_into_input_type_str(Self: Self) -> Option<Self::InputTypeStr> {
        None
    }
}

impl<S: KnownStr> InputType for S {
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
