pub use event_listener::OnEvent;
pub use frender_csr::{
    event_listener::{EventListenerState, HandleEvent, MaybeHandleEvent, RegisterOrUpdate},
    render_state, RenderState,
};
pub use frender_events::{event, event_types, HasEventTypeName};
pub use render_state_with_peh::{RenderStateWithAnyParent, RenderStateWithParentElementsHandle};

pub mod render;

pub mod csr;

pub mod component;

pub mod behaviors;

pub mod script;

pub mod attr;

pub mod special;

pub mod node_ref;

pub mod event_listener;

pub mod form_control;

mod render_state_with_peh;

#[cfg(feature = "web")]
mod shims;
