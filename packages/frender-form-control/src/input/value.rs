use crate::FormControlValueKind;

use super::InputValueKind;

#[cfg(feature = "csr")]
pub(super) mod csr;
#[cfg(feature = "ssr")]
pub(super) mod ssr;

mod imp;

pub trait InputValue {
    type ValueKind: ?Sized + FormControlValueKind + InputValueKind;
}
