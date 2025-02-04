#![cfg_attr(feature = "macros_not_expanded", recursion_limit = "2048")]

pub use frender_common::expand;
pub use frender_dom as dom;
pub use frender_dom::dom_tokens::{DomToken, DomTokenList, DomTokens};

pub use self::dom::Empty;

pub use html::RenderHtml;
pub use update_element::{BehaviorType, UiHandleType};

use element::CsrElement as Element; // TODO: remove

// TODO: some apis are unstable and should be sealed
pub use element::{CsrElement, HtmlRenderContext, RenderStateKind};
pub use element_types::{CsrComponent, CsrComponentNormalElement};
pub use frender_dom::{render_state, RenderStateWithParentElementsHandle, StateUnmount};

pub use frender_form_control as form_control;

pub mod html;

pub use html::components as cs;

pub mod stateless_render;

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

#[doc(hidden)]
/// This is not public api.
/// See also mod [`experimental`](crate::experimental) for experimental api under a feature.
pub mod __private {
    pub use crate::element::{PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind};
}

mod special;

mod attr;

mod attr_value;
mod dom_tokens;
mod event_listener;
mod property_common;
mod style;

mod impl_bounds;
use impl_bounds::impl_bounds;

pub mod intrinsic;

// TODO: feature
pub mod experimental;

mod utils;
