#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
pub use macros::*;

pub mod omitted;

mod static_text;
pub use static_text::*;
