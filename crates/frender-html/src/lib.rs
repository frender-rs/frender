pub use frender_common::expand;
pub use frender_dom as dom;
pub use frender_html_common::{dom_token::DomTokenList, DomTokens};

pub use create_node::CreateNode;
pub use html::RenderHtml;
pub use update_element::UpdateElementNonReactive;

pub use element::Element;
pub use element_types::{CsrComponent, CsrComponentNormalElement};
pub use frender_dom::{render_state, RenderState};

pub mod html;
pub mod impl_bounds;

mod create_node;
mod update_element;

mod element;

mod element_types;

pub mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

// TODO(refactor)
pub mod __private {
    pub use crate::RenderHtml;
}

mod special;
