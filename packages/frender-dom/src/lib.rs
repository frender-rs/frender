pub use event_listener::OnEvent;
pub use frender_common::Empty;
pub use frender_csr::{
    event_listener::{HandleEvent, MaybeHandleEvent, PinnedRegisterUpdate, RegisterUpdate},
    render_state, RenderState, StateUnmount,
};
pub use frender_events::{event, event_types, HasEventTypeName};
pub use provide_render_context::ProvideRenderContext;
pub use render_state_with_peh::{RenderStateWithAnyParent, RenderStateWithParentElementsHandle};

pub mod style {
    pub mod csr {
        pub use frender_style::csr::{CssStyleDeclaration, Priority};
    }
}

pub mod dom_tokens {
    pub use frender_dom_tokens::{DomToken, DomTokenList, DomTokens};
}

pub mod render;

pub mod csr;

pub mod component;

pub mod behaviors;

pub mod ui_handle;

pub mod script;

pub use frender_attr_value as attr_value;

pub mod special;

pub mod node_ref;

pub mod event_listener;

pub mod string_element;

pub mod render_from;

mod provide_render_context;
mod render_state_with_peh;

#[cfg(feature = "web")]
mod shims;
