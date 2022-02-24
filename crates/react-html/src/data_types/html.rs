use std::borrow::Cow;

use convert_js::ToJs;

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "camelCase")]
pub enum Inheritable<T> {
    Value(T),
    Inherit,
}

/// Hints at the type of data that might be entered by the user while editing the element or its contents
///
/// @see https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute
///
/// 'none' | 'text' | 'tel' | 'url' | 'email' | 'numeric' | 'decimal' | 'search'
#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "camelCase")]
pub enum HtmlInputMode {
    None,
    Text,
    Tel,
    Url,
    Email,
    Numeric,
    Decimal,
    Search,
}

#[derive(Debug, Clone, ToJs)]
#[convert_js(union)]
pub enum AnchorTarget {
    #[convert_js(rename = "_self")]
    SelfTarget,
    #[convert_js(rename = "_blank")]
    Blank,
    #[convert_js(rename = "_parent")]
    Parent,
    #[convert_js(rename = "_top")]
    Top,
    Custom(String),
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "kebab-case")]
pub enum ReferrerPolicy {
    #[convert_js(rename = "")]
    None,
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

#[derive(Debug, Clone, Copy, ToJs)]
#[convert_js(union, rename_all = "lowercase")]
pub enum HtmlLoading {
    Eager,
    Lazy,
}
