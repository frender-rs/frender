#[cfg(feature = "hooks")]
pub mod hooks_ext;

pub mod elements;

#[cfg(feature = "ToElement")]
mod to_element;
#[cfg(feature = "ToElement")]
pub use to_element::{ToElement, ToElementWithFn};

#[cfg(feature = "ToElement")]
pub use elements::synced_elements::{
    IteratorOfToElement, SyncedElementCollection, SyncedElements, ToIterOfToElement,
};

mod empty;
pub use empty::Empty;

mod missing;
pub use missing::Missing;

pub use frender_common::{EventListenerOptions, HandleEventWithOptions, TempStr};
pub use frender_hook_element::new_fn_hook_element;
pub use frender_html as html;
pub use frender_html::dom;
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
pub use frender_events::event;

#[cfg(feature = "bg")]
pub use bg;

#[cfg(feature = "RenderWith")]
pub use frender_render_with::{
    CsrRenderContext, DefaultAnyRenderState, FnOnceRenderWithContext, IntoFnOnceRenderWithContext,
    RenderWith, Rendered,
};

#[cfg(feature = "Elements")]
pub use frender_elements::{DefaultElementsAlgorithm, Elements, Keyed};

pub use frender_html::dom::script::ScriptInnerTextWronglyEncoded;
pub use frender_html::dom::special::DangerousInnerHtml;

pub use frender_html_common::dom_tokens;
pub use frender_html_common::dom_tokens::{
    dom_tokens, impl_dom_tokens_for, proxy_chainable_dom_tokens, proxy_dom_tokens,
    ChainableDomTokens, DomTokenList, DomTokens,
};

#[cfg(all(feature = "web"))]
pub use frender_csr_web::mount::GetDomElement;

pub mod main {
    #[cfg(feature = "web")]
    pub use frender_csr_web::mount::mount_to_dom_element;

    #[cfg(all(feature = "web", feature = "spawn"))]
    pub use frender_csr_web::mount::spawn_mount_to_dom_element;
}

pub mod prelude {
    #[cfg(feature = "bg")]
    pub use bg::{Maybe as _, MaybeBorrow as _};

    // #[cfg(all(feature = "csr", feature = "ssr"))]
    // pub use crate::Element;

    pub use crate::rsx;

    pub use frender_html::dom::{HandleEvent, MaybeHandleEvent};

    pub use frender_hook_element::component_fn;

    pub use frender_html::Element;

    #[cfg(feature = "Elements")]
    pub use crate::{Elements, Keyed};

    pub use frender_html_common::dom_tokens::{dom_tokens, ChainableDomTokens, DomTokens};

    pub use crate::elements;

    pub use frender_macros::component;

    #[cfg(feature = "html-components")]
    pub use frender_html::{cs, cs as intrinsic_components, prelude_props_builders::*};

    #[cfg(feature = "hooks")]
    pub use crate::hooks_ext::ShareValueExt;

    // #[cfg(feature = "csr")]
    // pub use frender_hook_element::frender_csr::{
    //     CsrContext, CsrElement, CsrRenderState, ElementsLinkedVec,
    // };

    // #[cfg(feature = "ssr")]
    pub use frender_ssr::{SsrElement, SsrElementExt};
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
    pub use frender_hook_element::__private::hooks_core;

    pub use frender_macros::rsx as impl_rsx;
}

#[macro_export]
macro_rules! elements {
    ($t0:expr, $t1:expr, $t2:expr, $t3:expr, $t4:expr, $t5:expr, $t6:expr, $t7:expr, $t8:expr, $t9:expr, $t10:expr, $t11:expr, $($t:tt)+) => {
        (
            (
                $t0, $t1, $t2, $t3, $t4, $t5, $t6, $t7, $t8, $t9, $t10, $t11,
            ),
            $crate::elements! { $($t)+ },
        )
    };
    ($($t0:expr),+ $(,)?)=>{
        ($($t0,)+)
    };
}
