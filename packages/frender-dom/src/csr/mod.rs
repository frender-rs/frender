pub use frender_csr_core::{
    event_listener::{HandleEvent, MaybeHandleEvent, PinnedRegisterUpdate, RegisterUpdate},
    StateUnmount,
};

#[cfg(feature = "web")]
pub mod web;

mod event_listener;
pub use event_listener::OnEvent;

mod provide_render_context;
pub use provide_render_context::ProvideRenderContext;

pub mod render;
pub mod render_from;

pub mod behaviors;

mod ui_handle;
pub use ui_handle::{ProvideMutMounted, UiHandle, UnmountedUiHandle};
