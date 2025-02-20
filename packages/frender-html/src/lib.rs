#![cfg_attr(feature = "macros_not_expanded", recursion_limit = "2048")]

pub use frender_common::{csr::StateUnmount, expand};
pub use frender_dom as dom;
pub use frender_dom::dom_tokens::{DomToken, DomTokenList, DomTokens};

pub use self::dom::Empty;

#[cfg(feature = "csr")]
pub use html::RenderHtml;
#[cfg(feature = "csr")]
pub use update_element::{BehaviorType, UiHandleType};

// TODO: some apis are unstable and should be sealed
#[cfg(feature = "csr")]
pub use element::{CsrElement, HtmlRenderContext, RenderStateKind};
#[cfg(feature = "csr")]
pub use element_types::{CsrComponent, CsrComponentNormalElement};

pub use frender_form_control as form_control;

// TODO: make private
pub mod html;

#[cfg(feature = "components")]
pub use html::components as cs;

mod has_const_attr_name;
mod into_property;

#[cfg(feature = "csr")]
pub mod stateless_render;

#[cfg(feature = "ElementProxyAttrs")]
pub mod element_proxy_attrs;

#[cfg(feature = "ElementProxyAttrs")]
pub use element_proxy_attrs::ElementProxyAttrs;

#[cfg(feature = "csr")]
mod update_element;

#[cfg(feature = "csr")]
mod element;

#[cfg(feature = "csr")]
mod element_types;

#[cfg(feature = "csr")]
pub mod kinds;

// TODO: move to frender-dom or separate crates
#[cfg(feature = "csr")]
pub mod ui_handles;

pub mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

#[doc(hidden)]
/// This is not public api.
/// See also mod [`experimental`](crate::experimental) for experimental api under a feature.
pub mod __private {
    #[cfg(feature = "csr")]
    pub use crate::element::{PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind};
}

mod special;

mod attr_value;
mod dom_tokens;
mod event_listener;
#[cfg(feature = "csr")]
mod property_common;
mod style;

mod impl_bounds;
use impl_bounds::impl_bounds;

pub mod intrinsic;

// TODO: move under mod csr
#[cfg(feature = "csr")]
#[cfg(feature = "experimental")]
pub mod experimental;

#[cfg(feature = "csr")]
mod utils;

#[cfg(feature = "csr")]
pub mod csr {
    pub use crate::html::RenderHtml;

    pub use crate::element::{CsrElement, HtmlRenderContext};

    #[cfg(feature = "experimental")]
    pub use super::experimental;

    pub use super::kinds;

    pub use frender_common::csr::StateUnmount;

    pub use frender_dom::csr::{UiHandle, UnmountedUiHandle};

    pub mod render {
        pub use frender_dom::csr::render::{RenderContext, RenderWithContext};
    }

    // TODO: move to frender-dom or separate crates
    pub mod ui_handles {
        pub use crate::ui_handles::CursorPlaceholdersSurrounded;
    }
}
