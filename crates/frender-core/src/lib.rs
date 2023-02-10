#![deny(clippy::undocumented_unsafe_blocks)]

mod render_state;
mod update_render_state;

pub use render_state::*;
pub use update_render_state::*;

mod static_text;
pub use static_text::*;

mod elements;
pub use elements::*;

mod utils;

pub use utils::LazyPinned;
