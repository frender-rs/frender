mod debug;
pub use debug::*;

#[cfg(all(feature = "csr", feature = "ssr"))]
mod element;
#[cfg(all(feature = "csr", feature = "ssr"))]
pub use element::*;

#[cfg(feature = "hooks")]
pub mod hooks_ext;

mod element_macros;

pub mod elements;
pub mod omitted;

pub use elements::static_text::StaticText;

pub use frender_common::{Elements, Keyed};
pub use frender_macros::component;

// #[cfg(feature = "csr")]
// pub use frender_hook_element::frender_csr as csr;

// #[cfg(feature = "ssr")]
// pub use frender_hook_element::frender_ssr as ssr;

pub use frender_ssr as ssr;

pub use prelude::*;

// #[cfg(feature = "html")]
// pub mod html {
//     // pub use frender_html::*;

//     #[cfg(feature = "html-components")]
//     pub use frender_html::html::{components, components as intrinsic_components};
// }

pub use frender_hook_element as hook_element;
pub use frender_hook_element::component_fn;

pub use event::*;
pub use frender_events::{event, MaybeHandleEvent};

#[cfg(feature = "bg")]
pub use bg;

pub mod prelude {
    #[cfg(feature = "bg")]
    pub use bg::{Maybe as _, MaybeBorrow as _};

    // #[cfg(all(feature = "csr", feature = "ssr"))]
    // pub use crate::Element;

    pub use crate::{rsx, StaticText};

    pub use frender_hook_element::component_fn;

    pub use frender_element::Element;

    pub use frender_common::{Elements, Keyed};

    pub use crate::{elements, intrinsic};

    pub use frender_macros::component;

    pub use frender_events::callable::prelude::*;

    #[cfg(feature = "html-components")]
    pub use frender_html::html::components as intrinsic_components;

    #[cfg(feature = "hooks")]
    pub use crate::hooks_ext::ShareValueExt;

    // #[cfg(feature = "csr")]
    // pub use frender_hook_element::frender_csr::{
    //     CsrContext, CsrElement, CsrRenderState, ElementsLinkedVec,
    // };

    // #[cfg(feature = "ssr")]
    // pub use frender_hook_element::frender_ssr::{SsrElement, SsrElementExt, SsrRenderState};
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
