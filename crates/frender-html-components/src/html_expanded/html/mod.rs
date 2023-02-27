#![allow(non_camel_case_types)]
use frender_html::props::events;
#[cfg(feature = "html_macro_not_expand")]
pub mod element_macros;
#[cfg(feature = "fully-typed")]
pub mod fully_typed;
