#![deny(clippy::undocumented_unsafe_blocks)]

pub use frender_csr::{RenderState, UpdateRenderState}; // TODO: remove

mod macros;
pub use macros::*;

pub mod omitted;

mod static_text;
pub use static_text::*;
