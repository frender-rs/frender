#[cfg(any(feature = "ssr", feature = "dom"))]
pub mod component_macro;

mod ctx_and_state;
mod hook_context;

pub use ctx_and_state::*;
pub use hook_context::*;

mod hook_with_no_props;
mod hook_with_owned_props;
mod hook_with_ref_props;

pub use hook_with_no_props::*;
pub use hook_with_owned_props::*;
pub use hook_with_ref_props::*;

#[cfg(feature = "dom")]
mod dom;

#[cfg(feature = "ssr")]
mod ssr;