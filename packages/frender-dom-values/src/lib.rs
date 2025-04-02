pub use frender_dom::{
    //
    attrs::Attrs,
    special::DangerousInnerHtml,
    string_element::StringElement,
    Empty,
};

#[cfg(feature = "csr")]
pub use frender_dom::script::ScriptInnerTextCsrOnly;

#[cfg(feature = "ssr")]
pub use frender_dom::script::ScriptInnerTextWronglyEncoded;

pub mod form_control {
    pub use frender_form_control::values::{
        //
        EitherFormControlValue,
        UncontrolledWithDefaultValue,
    };

    pub mod input {
        pub use frender_form_control::input::{
            InputDataModel, IntoInputDataModel, {InputChecked, InputType, InputValue},
        };

        #[cfg(feature = "csr")]
        pub use frender_form_control::input::{
            IntoCsrInputDataModel, {CsrInputChecked, CsrInputType, CsrInputValue},
        };
        #[cfg(feature = "ssr")]
        pub use frender_form_control::input::{
            IntoSsrInputDataModel, {SsrInputChecked, SsrInputType, SsrInputValue},
        };
    }

    pub mod textarea {
        pub use frender_form_control::textarea::TextAreaValue;

        #[cfg(feature = "csr")]
        pub use frender_form_control::textarea::CsrTextAreaValue;
        #[cfg(feature = "ssr")]
        pub use frender_form_control::textarea::SsrTextAreaValue;
    }
}

#[doc(no_inline)]
pub use self::macros::attr_value;
pub mod attr_value {
    pub use frender_attr_value::{
        values::{
            //
            Absent,
            EitherAttrValue,
        },
        AttrKindOfStr, AttrValue, AttrValueKind, IntoAttrValue,
    };

    #[cfg(feature = "html")]
    pub use frender_attr_value::html::{AttrKindOfContentEditable, AttrKindOfSpellcheck};

    pub use frender_attr_value::{
        attr_value::{one, r#const},
        impl_HasConstAttrValue_for,
    };
}

#[doc(no_inline)]
pub use self::macros::attrs;
pub mod attrs {
    pub use frender_attrs::values::{
        //
        r#const::{ConstAttributes, HasConstAttributes},
        Chain,
        EitherAttributes,
        Never,
    };

    pub use frender_attrs::{Attributes, IntoAttributes};

    pub use frender_attrs::{
        attrs::{comma_separated, one, r#const},
        impl_HasConstAttributes_for,
    };
}

#[doc(no_inline)]
pub use self::macros::style;
pub mod style {
    pub use frender_style::{
        impl_has_const_declaration_list_for,
        style::{comma_separated, one, r#const},
        styles::{constness::ConstDeclarationList, Chain, EitherStyle, Never},
        IntoStyle, Style,
    };
}

#[doc(no_inline)]
pub use self::macros::dom_tokens;
pub mod dom_tokens {
    pub use frender_dom_tokens::{
        //
        constness::{ConstDomTokens, HasConstDomTokens},
        dom_tokens::{comma_separated, one, r#const},
        impl_dom_tokens_for,
        impl_has_const_dom_tokens_for,
        values::{Chain, EitherDomTokens, EraseConstKnownPossibleDomTokens},
        {ChainableDomTokens, DomTokens, IntoDomTokens},
    };
}

pub mod macros {
    pub use frender_attr_value::attr_value::one as attr_value;
    pub use frender_attrs::attrs::comma_separated as attrs;
    pub use frender_dom_tokens::dom_tokens::comma_separated as dom_tokens;
    pub use frender_style::style::comma_separated as style;
}

pub mod event {
    pub use frender_events::event::{
        AnimationEvent, CompositionEvent, Event, FocusEvent, InputEvent, KeyboardEvent, MouseEvent,
        PointerEvent, SecurityPolicyViolationEvent, TouchEvent, TransitionEvent, UiEvent,
        WheelEvent,
    };
}
