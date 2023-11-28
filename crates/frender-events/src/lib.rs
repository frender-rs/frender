// TODO: move
mod event_trait;
pub use event_trait::*;

// TODO: move
pub use callable;
use frender_common::expand;

pub mod event;
pub mod event_types;
pub mod touch;
#[cfg(feature = "web")]
pub mod web;

pub trait HasEventTypeName {
    const EVENT_TYPE_NAME: &'static str;
}
