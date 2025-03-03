use frender_common::Empty;

use crate::{AttrValue, AttrValueKind};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl AttrValueKind for bool {
    /// Boolean attributes doesn't have value.
    type AttrValue<'a> = ();
}

impl AttrValue<bool> for Empty {}
impl AttrValue<bool> for bool {}
