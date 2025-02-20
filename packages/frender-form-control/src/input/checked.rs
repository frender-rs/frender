use frender_dom::Empty;

use crate::values::{EitherFormControlValue, UncontrolledWithDefaultValue};

#[cfg(feature = "csr")]
pub(super) mod csr;
#[cfg(feature = "ssr")]
pub(super) mod ssr;

pub trait InputChecked {}

impl InputChecked for Empty {}
impl InputChecked for bool {}
impl<T> InputChecked for UncontrolledWithDefaultValue<T> {}

impl<T: InputChecked> InputChecked for Option<T> {}
impl<A: InputChecked, B: InputChecked> InputChecked for EitherFormControlValue<A, B> {}

#[cfg(feature = "either")]
impl<L: InputChecked, R: InputChecked> InputChecked for either::Either<L, R> {}
