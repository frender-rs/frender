// pub use frender_element::Element;

#[cfg(feature = "hooks_ext")]
pub mod hooks_ext;

pub mod elements;

#[cfg(feature = "ToElement")]
pub use frender_to_element::{RefToElementWithFn, ToElement, ToElementWithFn};

#[cfg(feature = "SyncedCollection")]
pub use elements::synced_collection::{SyncedCollection, SyncedVec};

pub use frender_common::{either::EitherElement, EventListenerOptions, HandleEventWithOptions};
pub use frender_hook_element::new_fn_hook_element;
pub use frender_macros::component;
pub use frender_reactive_value::{
    non_reactive::Uncached, static_or_temp_ref::StaticOrTempRef, temp_into_static::TempIntoStatic,
    temp_ref::TempRef,
};

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

#[cfg(feature = "RenderWith")]
pub use frender_render_with::{self as render_with, RenderWith};

#[cfg(feature = "KeyedElements")]
pub use frender_keyed_elements::{Keyed, KeyedElements};

#[cfg(feature = "KeyedElements")]
pub use frender_keyed_elements as keyed_elements;

#[cfg(feature = "context")]
pub use frender_context as context;
#[cfg(feature = "context")]
pub use frender_context::{
    local::{LocalContextKeyProvided, LocalContextKeyUnprovided},
    local_context,
};

#[cfg(feature = "Memo")]
pub use frender_memo::Memo;

#[cfg(all(feature = "web"))]
pub use frender_csr_web::mount::GetDomElement;

pub mod main {
    #[cfg(feature = "web")]
    pub use frender_csr_web::mount::mount_to_dom_element;

    #[cfg(all(feature = "web", feature = "spawn"))]
    pub use frender_csr_web::mount::spawn_mount_to_dom_element;
}

// region: values
pub use frender_html_values::{
    //
    DangerousInnerHtml,
    Empty,
    Intrinsic,
    StringElement,
};

#[cfg(feature = "csr")]
pub use frender_html_values::ScriptInnerTextCsrOnly;

#[cfg(feature = "ssr")]
pub use frender_html_values::ScriptInnerTextWronglyEncoded;

pub mod attr_value {
    pub use frender_html_values::attr_value::{
        //
        Absent,
        // html
        AttrKindOfContentEditable,
        AttrKindOfStr,
        AttrValue,
        AttrValueKind,
        EitherAttrValue,
        Spellcheck,
    };

    #[cfg(feature = "csr")]
    pub use frender_html_values::attr_value::CsrAttrValue;
    #[cfg(feature = "ssr")]
    pub use frender_html_values::attr_value::SsrAttrValue;
}

#[doc(no_inline)]
pub use self::macros::dom_tokens;
pub mod dom_tokens {
    pub use frender_html_values::dom_tokens::{
        impl_dom_tokens_for,
        impl_has_const_dom_tokens_for,
        Chain,
        EitherDomTokens,
        EraseConstKnownPossibleDomTokens,
        {comma_separated, one, r#const},
        //
        {ConstDomTokens, HasConstDomTokens},
        {DomTokens, IntoDomTokens},
    };
}

pub mod form_control {
    pub use frender_html_values::form_control::{
        //
        EitherFormControlValue,
        UncontrolledWithDefaultValue,
    };

    pub mod input {
        pub use frender_html_values::form_control::input::{
            InputDataModel, IntoInputDataModel, {InputChecked, InputType, InputValue},
        };

        #[cfg(feature = "csr")]
        pub use frender_html_values::form_control::input::{
            IntoCsrInputDataModel, {CsrInputChecked, CsrInputType, CsrInputValue},
        };
        #[cfg(feature = "ssr")]
        pub use frender_html_values::form_control::input::{
            IntoSsrInputDataModel, {SsrInputChecked, SsrInputType, SsrInputValue},
        };
    }

    pub mod textarea {
        pub use frender_html_values::form_control::textarea::TextAreaValue;

        #[cfg(feature = "csr")]
        pub use frender_html_values::form_control::textarea::CsrTextAreaValue;
        #[cfg(feature = "ssr")]
        pub use frender_html_values::form_control::textarea::SsrTextAreaValue;
    }
}

#[doc(no_inline)]
pub use self::macros::style;
pub mod style {
    pub use frender_style::{
        css_style_declaration, impl_has_const_declaration_list_for,
        style::{comma_separated, one, r#const},
        styles::{constness::ConstDeclarationList, Chain, EitherStyle, Never},
        IntoStyle, Style,
    };

    #[cfg(feature = "csr")]
    pub use frender_style::csr::CsrStyle;
    #[cfg(feature = "ssr")]
    pub use frender_style::ssr::SsrStyle;
}

pub mod macros {
    pub use frender_html_values::macros::{dom_tokens, style};
}
// endregion

pub mod prelude {
    // #[cfg(all(feature = "csr", feature = "ssr"))]
    // pub use crate::Element;

    pub use crate::rsx;

    pub use frender_common::{HandleEvent, MaybeHandleEvent};

    pub use frender_hook_element::component_fn;

    pub use frender_element::CsrElement;

    pub use frender_element::Element;

    #[cfg(feature = "KeyedElements")]
    pub use crate::{Keyed, KeyedElements};

    pub use crate::macros::{dom_tokens, style};

    pub use crate::elements;

    pub use frender_macros::component;

    #[cfg(feature = "html-components")]
    pub use {cs as intrinsic_components, frender_html_components as cs};

    #[cfg(feature = "hooks_ext")]
    pub use crate::hooks_ext::ShareValueExt as _;

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
