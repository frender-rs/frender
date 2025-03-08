pub use frender_dom::{
    //
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

pub mod attr_value {
    pub use frender_attr_value::{
        values::{
            //
            Absent,
            EitherAttrValue,
        },
        AttrKindOfStr, AttrValue, AttrValueKind,
    };

    #[cfg(feature = "html")]
    pub use frender_attr_value::html::{AttrKindOfContentEditable, AttrKindOfSpellcheck};

    #[cfg(feature = "csr")]
    pub use frender_attr_value::csr::CsrAttrValue;
    #[cfg(feature = "ssr")]
    pub use frender_attr_value::ssr::SsrAttrValue;
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

#[doc(no_inline)]
pub use self::macros::dom_tokens;
pub mod dom_tokens {
    pub use frender_dom_tokens::{
        //
        constness::{ConstDomTokens, HasConstDomTokens},
        dom_tokens::{comma_separated, one, r#const},
        impl_dom_tokens_for,
        impl_has_const_dom_tokens_for,
        Chain,
        EitherDomTokens,
        EraseConstKnownPossibleDomTokens,
        {ChainableDomTokens, DomTokens, IntoDomTokens},
    };
}

pub mod macros {
    pub use frender_dom_tokens::dom_tokens::comma_separated as dom_tokens;
    pub use frender_style::style::comma_separated as style;
}
