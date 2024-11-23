#![recursion_limit = "2048"]
pub use frender_common::expand;
pub use frender_dom as dom;
pub use frender_dom::dom_tokens::{DomToken, DomTokenList, DomTokens};

pub use self::dom::Empty;

pub use create_node::CreateNode;
pub use html::RenderHtml;
pub use update_element::{BehaviorType, UpdateNodeNonReactive, UpdateNodeNonReactivePinned};

use element::CsrElement as Element; // TODO: remove

pub use element::{CsrElement, HtmlRenderContext, RenderStateKind, RenderStateKindPinned, RenderStateKindUnpinned, RenderStateOfContext, UnpinnedRenderStateOfContext};
pub use element_types::{CsrComponent, CsrComponentNormalElement};
pub use frender_dom::{form_control, render_state, RenderState, RenderStateWithParentElementsHandle};

pub mod html;

pub use html::components as cs;

#[cfg(feature = "ElementProxyAttrs")]
pub mod element_proxy_attrs;

#[cfg(feature = "ElementProxyAttrs")]
pub use element_proxy_attrs::ElementProxyAttrs;

mod create_node;
mod update_element;

mod element;

mod element_types;

pub mod kinds;

pub mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

// TODO(refactor)
pub mod __private {
    pub use crate::RenderHtml;
}

mod special;

mod attr;

mod impl_bounds;
use impl_bounds::impl_bounds;

pub mod intrinsic;
