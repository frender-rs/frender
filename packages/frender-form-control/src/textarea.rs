use frender_common::Empty;

use crate::{
    known_str::KnownIsNonReactiveStrOrUncached,
    values::{EitherFormControlValue, UncontrolledWithDefaultValue},
};

#[cfg(feature = "csr")]
pub use self::csr::CsrTextAreaValue;
#[cfg(feature = "ssr")]
pub use self::ssr::SsrTextAreaValue;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

pub trait TextAreaValue {}

impl TextAreaValue for Empty {}
impl<T: KnownIsNonReactiveStrOrUncached> TextAreaValue for T {}
impl<T: TextAreaValue> TextAreaValue for Option<T> {}
impl<T> TextAreaValue for UncontrolledWithDefaultValue<T> {}
impl<A: TextAreaValue, B: TextAreaValue> TextAreaValue for EitherFormControlValue<A, B> {}

#[cfg(feature = "either")]
impl<L: TextAreaValue, R: TextAreaValue> TextAreaValue for either::Either<L, R> {}
