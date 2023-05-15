#![allow(non_camel_case_types)]
use frender_events::events;
#[cfg(feature = "fully-typed")]
pub mod fully_typed;
#[cfg(feature = "simply-typed")]
pub mod simply_typed;
