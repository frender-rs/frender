use frender_common::Empty;

use crate::Style;

impl crate::sealed::Style for Empty {}
impl Style for Empty {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
