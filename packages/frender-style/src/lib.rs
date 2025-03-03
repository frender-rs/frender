#[cfg(feature = "csr")]
pub mod csr;
#[cfg(feature = "ssr")]
pub mod ssr;

mod declaration;

pub mod css_style_declaration;
pub mod styles;

#[cfg(feature = "web")]
mod web;

/// Anything that can be used as html style attribute.
///
/// This is actually a [Declaration](declaration::Declaration) list.
///
/// https://drafts.csswg.org/css-style-attr/#syntax
/// https://w3c.github.io/csswg-drafts/css-style-attr/#syntax
pub trait Style {}

pub trait IntoStyle {
    type IntoStyle: Style;

    fn into_style(self) -> Self::IntoStyle;
}

impl<T: ?Sized + IntoStyle> Style for T {}

pub mod style;
