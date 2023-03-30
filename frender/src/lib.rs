mod debug;
pub use debug::*;

mod element_macros;

pub mod elements;
pub mod omitted;

pub use elements::static_text::StaticText;

pub use frender_common::Keyed;
pub use frender_macros::{component, def_props};

#[cfg(feature = "csr")]
pub use frender_hook_element::frender_csr::{CsrContext, Element as CsrElement};

#[cfg(feature = "ssr")]
pub use frender_hook_element::frender_ssr::{Element as SsrElement, ElementExt as SsrElementExt};

#[cfg(feature = "html")]
pub mod html {
    pub use frender_html::*;

    #[cfg(feature = "html-components")]
    pub use frender_html_components as components;

    #[cfg(feature = "html-components-simply-typed")]
    pub use frender_html_components::intrinsic_components;
}

#[cfg(feature = "html")]
pub use html::props::{events, UpdateDomEventListener};

pub use frender_hook_element as hook_element;
pub use frender_hook_element::{component_fn, Element};

#[cfg(feature = "bg")]
pub use bg;

pub mod prelude {
    #[cfg(feature = "bg")]
    pub use bg::{Maybe as _, MaybeBorrow as _};

    pub use crate::{rsx, StaticText};

    pub use frender_hook_element::{component_fn, Element};

    pub use frender_common::Keyed;

    pub use crate::{elements, intrinsic};

    pub use frender_macros::component;

    #[cfg(feature = "html-components-simply-typed")]
    pub use frender_html_components::intrinsic_components;
}

#[macro_export]
macro_rules! rsx {
    ($($rest:tt)*) => {
        // specify crate path is `$crate`
        $crate::__private::impl_rsx! {
            @[$crate]
            $($rest)*
        }
    };
}

#[doc(hidden)]
pub mod __private {
    pub use frender_hook_element;

    pub use frender_macros::rsx as impl_rsx;
}
