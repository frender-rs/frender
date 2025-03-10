#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

mod declaration;

pub mod css_style_declaration;
pub mod styles;

#[cfg(feature = "web")]
mod web;

mod sealed {
    #[cfg(feature = "csr")]
    use crate::csr::CsrStyle;
    #[cfg(feature = "ssr")]
    use crate::ssr::SsrStyle;

    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub trait Style: CsrStyle + SsrStyle {}

    #[cfg(feature = "csr")]
    #[cfg(not(feature = "ssr"))]
    pub trait Style: CsrStyle {}

    #[cfg(not(feature = "csr"))]
    #[cfg(feature = "ssr")]
    pub trait Style: SsrStyle {}

    #[cfg(not(feature = "csr"))]
    #[cfg(not(feature = "ssr"))]
    pub trait Style {}
}

/// Anything that can be used as html style attribute.
///
/// This is actually a [Declaration](declaration::Declaration) list.
///
/// https://drafts.csswg.org/css-style-attr/#syntax
/// https://w3c.github.io/csswg-drafts/css-style-attr/#syntax
pub trait Style: sealed::Style {}

pub trait IntoStyle {
    type IntoStyle: Style;

    fn into_style(self) -> Self::IntoStyle;
}

impl<T: IntoStyle> sealed::Style for T {}
impl<T: IntoStyle> Style for T {}

pub mod style;

pub mod declaration_name;
pub mod declaration_value;

#[cfg(feature = "experimental")]
pub mod experimental;
