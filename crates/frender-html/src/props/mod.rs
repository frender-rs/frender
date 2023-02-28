mod event;
mod maybe_inherit;
mod string;
mod style;

pub use event::*;
pub use maybe_inherit::*;
pub use string::*;
pub use style::*;

pub mod events;

pub use frender_html_common::IntrinsicComponent; //TODO: refactor
pub use frender_html_common::MaybeUpdateValueWithState; //TODO: refactor
