#![recursion_limit = "2048"]
pub use frender_common::expand;
pub use frender_dom as dom;
pub use frender_dom::dom_tokens::{DomToken, DomTokenList, DomTokens};

pub use self::dom::Empty;

pub use html::RenderHtml;
pub use update_element::{BehaviorType, UiHandleType, UpdateNodeNonReactive, UpdateNodeNonReactivePinned};

use element::CsrElement as Element; // TODO: remove

// TODO: some apis are unstable and should be sealed
pub use element::{CsrElement, HtmlRenderContext};
pub use element_types::{CsrComponent, CsrComponentNormalElement};
pub use frender_dom::{form_control, render_state, RenderStateWithParentElementsHandle, StateUnmount};

pub mod html;

pub use html::components as cs;

#[cfg(feature = "ElementProxyAttrs")]
pub mod element_proxy_attrs;

#[cfg(feature = "ElementProxyAttrs")]
pub use element_proxy_attrs::ElementProxyAttrs;

mod update_element;

mod element;

mod element_types;

pub mod kinds;
pub mod ui_handles;

pub mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

// TODO(refactor)
pub mod __private {
    pub use crate::RenderHtml;
}

#[cfg(todo)]
mod special;

mod attr;

mod impl_bounds;
use impl_bounds::impl_bounds;

pub mod intrinsic;

// TODO: feature
pub mod experimental;
