#![deny(clippy::undocumented_unsafe_blocks)]

mod macros;
mod render_state;
mod update_render_state;

pub use macros::*;
pub use render_state::*;
pub use update_render_state::*;

mod static_text;
pub use static_text::*;

mod render_with_element;
pub use render_with_element::*;

mod elements;
pub use elements::*;

mod utils;
