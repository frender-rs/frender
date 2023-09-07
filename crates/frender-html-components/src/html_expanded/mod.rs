#![allow(non_camel_case_types)]
use frender_events::events;
use frender_html_simple::impl_bounds::{Css, DomTokens, MaybeContentEditable};
#[cfg(feature = "fully-typed")]
pub mod fully_typed;
#[cfg(feature = "simply-typed")]
pub mod simply_typed;
