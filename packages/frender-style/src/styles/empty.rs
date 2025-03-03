use frender_common::Empty;

use crate::Style;

impl Style for Empty {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
