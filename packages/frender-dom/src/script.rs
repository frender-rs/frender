#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "csr")]
pub use csr::{CsrScriptContent, ScriptInnerTextCsrOnly};

#[cfg(feature = "ssr")]
mod ssr;
#[cfg(feature = "ssr")]
pub use ssr::{ScriptInnerTextWronglyEncoded, SsrScriptContent};
