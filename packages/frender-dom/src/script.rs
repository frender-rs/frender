#[cfg(feature = "csr")]
pub use csr::{CsrScriptContent, ScriptInnerTextCsrOnly};
#[cfg(feature = "ssr")]
pub use ssr::{ScriptInnerTextWronglyEncoded, SsrScriptContent};

use frender_common::Empty;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

pub trait ScriptContent {}

impl ScriptContent for Empty {}
