#![cfg_attr(feature = "macros_not_expanded", recursion_limit = "2048")]

use frender_common::{expand, Empty};

mod html;

#[cfg(feature = "components")]
pub use html::components as cs;

mod has_const_attr_name;
mod into_property;

#[cfg(feature = "ElementProxyAttrs")]
pub mod element_proxy_attrs;

#[cfg(feature = "ElementProxyAttrs")]
pub use element_proxy_attrs::ElementProxyAttrs;

#[cfg(feature = "csr")]
mod elements;

mod macros;
#[cfg(feature = "web")]
mod shims;

#[doc(hidden)]
/// This is not public api.
/// See also mod [`experimental`](crate::experimental) for experimental api under a feature.
pub mod __private {
    #[cfg(feature = "csr")]
    #[doc(hidden)]
    pub use crate::{
        csr::element::{HtmlRenderContext, PinnedStateOfKind, PinnedUiHandleOfKind, PinnedUnmountedUiHandleOfKind, UnpinnedStateOfKind, UnpinnedUiHandleOfKind, UnpinnedUnmountedUiHandleOfKind},
        html::RenderHtml,
    };
}

mod special;

mod attr_value;
mod dom_tokens;
mod event_listener;
mod style;

mod impl_bounds;
use impl_bounds::impl_bounds;

mod intrinsic;
pub use intrinsic::Intrinsic;

#[cfg(feature = "csr")]
mod utils;

#[cfg(feature = "csr")]
pub mod csr {
    pub use crate::{proxy_csr_element, proxy_csr_element_render_update, proxy_csr_element_without_pinned_render_init};

    pub(crate) mod behavior_type;
    pub(crate) mod component;

    pub(crate) mod element;
    pub use element::{CsrElement, RenderStateKind};

    pub(crate) mod stateless_render;

    pub(crate) mod property_common;

    pub(crate) mod kinds;

    #[cfg(feature = "experimental")]
    pub mod experimental;

    // pub use super::kinds;

    pub use frender_common::csr::StateUnmount;

    pub use frender_dom::csr::{UiHandle, UnmountedUiHandle};

    pub mod render {
        pub use frender_dom::csr::render::{RenderContext, RenderWithContext};
    }
}

pub mod values;
