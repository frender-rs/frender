pub use event_listener::OnEvent;
pub use frender_common::Empty;
pub use frender_csr::{
    event_listener::{EventListenerState, HandleEvent, MaybeHandleEvent, RegisterOrUpdate},
    render_state, RenderState,
};
pub use frender_events::{event, event_types, HasEventTypeName};
pub use provide_render_context::ProvideRenderContext;
pub use render_state_with_peh::{RenderStateWithAnyParent, RenderStateWithParentElementsHandle};

pub mod style {
    pub mod csr {
        pub use frender_style::csr::{CssStyleDeclaration, Priority};
    }
}

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

pub mod string_element;

mod provide_render_context;
mod render_state_with_peh;

#[cfg(feature = "web")]
mod shims;
