#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
pub use frender_element::Element;

#[cfg(feature = "csr")]
pub use frender_element::CsrElement;

#[cfg(feature = "ssr")]
pub use frender_element::SsrElement;

#[cfg(feature = "hooks_ext")]
pub mod hooks_ext;

pub mod elements;

#[cfg(feature = "ToElement")]
pub use frender_to_element::{RefToElementWithFn, ToElement, ToElementWithFn};

#[cfg(feature = "SyncedCollection")]
pub use elements::synced_collection::{SyncedCollection, SyncedVec};

#[cfg(feature = "HookElement")]
#[doc(no_inline)]
pub use crate::elements::hook_element::component_fn;

pub use frender_common::{either::EitherElement, EventListenerOptions, HandleEventWithOptions};
pub use frender_macros::component;
pub use frender_reactive_value::{
    non_reactive::Uncached, static_or_temp_ref::StaticOrTempRef, temp_into_static::TempIntoStatic,
    temp_ref::TempRef,
};

pub use frender_ssr as ssr;

pub use prelude::*;

// #[cfg(feature = "html")]
// pub mod html {
//     // pub use frender_html::*;

//     #[cfg(feature = "html-components")]
//     pub use frender_html::html::{components, components as intrinsic_components};
// }

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
    #[cfg(feature = "csr")]
    #[cfg(feature = "web")]
    pub use frender_csr_web::mount::mount_to_dom_element;

    #[cfg(feature = "csr")]
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

#[doc(no_inline)]
pub use self::macros::attr_value;
pub mod attr_value {
    pub use frender_html_values::attr_value::{
        impl_HasConstAttrValue_for,
        one,
        Absent,
        // html
        AttrKindOfContentEditable,
        AttrKindOfSpellcheck,
        AttrKindOfStr,
        AttrValue,
        AttrValueKind,
        EitherAttrValue,
        IntoAttrValue,
    };
}

#[doc(no_inline)]
pub use self::macros::attrs;
pub mod attrs {
    pub use frender_html_values::attrs::{
        comma_separated, impl_HasConstAttributes_for, one, r#const, Attributes, Chain,
        ConstAttributes, EitherAttributes, HasConstAttributes, IntoAttributes, Never,
    };
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
        css_style_declaration, impl_HasConstDeclarationList_for,
        style::{comma_separated, one, r#const},
        styles::{constness::ConstDeclarationList, Chain, EitherStyle, Never},
        IntoStyle, Style,
    };
}

pub mod macros {
    pub use frender_html_values::macros::{attr_value, attrs, dom_tokens, style};
}
// endregion

pub mod prelude {
    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub use crate::Element;

    #[cfg(feature = "csr")]
    pub use crate::CsrElement;

    #[cfg(feature = "ssr")]
    pub use crate::SsrElement;

    pub use crate::rsx;

    pub use frender_common::{HandleEvent, MaybeHandleEvent};

    #[cfg(feature = "HookElement")]
    pub use crate::component_fn;

    #[cfg(feature = "KeyedElements")]
    pub use crate::{Keyed, KeyedElements};

    pub use crate::macros::{dom_tokens, style};

    pub use crate::elements;

    pub use frender_macros::component;

    #[cfg(feature = "html-components")]
    pub use {cs as intrinsic_components, frender_html_components as cs};

    #[cfg(feature = "hooks_ext")]
    pub use crate::hooks_ext::ShareValueExt as _;

    // #[cfg(feature = "ssr")]
    pub use frender_ssr::SsrElementExt;
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
    #[cfg(feature = "HookElement")]
    // #[cfg(feature = "proc-macro")]
    #[doc(hidden)]
    pub use frender_hook_element::__private::hooks_core;

    #[doc(hidden)]
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
