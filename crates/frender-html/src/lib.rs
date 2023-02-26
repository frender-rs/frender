#[cfg(feature = "html_macro_not_expand")]
mod element_macros;
#[cfg(feature = "html_macro_not_expand")]
pub use element_macros::*;

#[cfg(feature = "html_macro_not_expand")]
use frender_macros::def_intrinsic_component_props;

mod data_types;
pub use data_types::*;

pub mod props;

mod utils;
