#[cfg(any(feature = "ssr", feature = "dom"))]
pub mod component_macro;

pub use bg;
pub use frender_core;

mod ctx_and_state;
mod hook_context;
mod no_data;

pub use ctx_and_state::*;
pub use hook_context::*;
pub use no_data::*;

mod hook_with_owned_props;
mod hook_with_ref_props;

pub use hook_with_owned_props::*;
pub use hook_with_ref_props::*;

#[cfg(feature = "dom")]
pub use frender_dom;

#[cfg(feature = "ssr")]
pub use frender_ssr;
