mod debug;
pub use debug::*;

mod element;
pub use element::*;

#[cfg(feature = "hooks")]
pub mod hooks_ext;

mod element_macros;

pub mod elements;
pub mod omitted;

pub use elements::static_text::StaticText;

pub use frender_common::{Elements, Keyed};
pub use frender_macros::{component, def_props};

#[cfg(feature = "csr")]
pub use frender_hook_element::frender_csr as csr;

#[cfg(feature = "ssr")]
pub use frender_hook_element::frender_ssr as ssr;

pub use prelude::*;

#[cfg(feature = "html")]
pub mod html {
    pub use frender_html::*;

    #[cfg(feature = "html-components")]
    pub use frender_html_components as components;

    #[cfg(feature = "html-components-simply-typed")]
    pub use frender_html_components::intrinsic_components;
}

pub use frender_hook_element as hook_element;
pub use frender_hook_element::{component_fn, RenderWith};

pub use frender_events::{events, MaybeHandleEvent};

#[cfg(feature = "bg")]
pub use bg;

pub mod prelude {
    #[cfg(feature = "bg")]
    pub use bg::{Maybe as _, MaybeBorrow as _};

    pub use crate::{rsx, Element, StaticText};

    pub use frender_hook_element::{component_fn, RenderWith};

    pub use frender_common::{Elements, Keyed};

    pub use crate::{elements, intrinsic};

    pub use frender_macros::component;

    pub use frender_events::callable::prelude::*;

    #[cfg(feature = "html-components-simply-typed")]
    pub use frender_html_components::intrinsic_components;

    #[cfg(feature = "hooks")]
    pub use crate::hooks_ext::ShareValueExt;

    #[cfg(feature = "csr")]
    pub use frender_hook_element::frender_csr::{
        CsrContext, CsrElement, CsrRenderState, ElementsLinkedVec,
    };

    #[cfg(feature = "ssr")]
    pub use frender_hook_element::frender_ssr::{SsrElement, SsrElementExt, SsrRenderState};
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
