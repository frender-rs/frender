mod debug;
pub use debug::*;

pub use frender_core::{element, intrinsic, Keyed, RenderState, StaticText, UpdateRenderState};
pub use frender_macros::{component, def_props};

#[cfg(feature = "dom")]
pub use frender_hook_element::frender_dom::Dom;

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

    pub use crate::rsx;

    pub use frender_hook_element::{component_fn, Element};

    pub use frender_core::{element, intrinsic, Keyed, StaticText};
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
