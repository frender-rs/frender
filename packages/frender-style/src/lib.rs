pub use frender_common::Empty;

pub mod csr;
pub mod ssr;

pub mod constness;

mod declaration;

pub mod styles;

#[cfg(feature = "web")]
mod web;

/// Anything that can be used as html style attribute.
///
/// This is actually a [Declaration](declaration::Declaration) list.
///
/// https://drafts.csswg.org/css-style-attr/#syntax
/// https://w3c.github.io/csswg-drafts/css-style-attr/#syntax
pub trait Style: csr::CsrStyle + ssr::SsrStyle {}

impl<S: ?Sized + csr::CsrStyle + ssr::SsrStyle> Style for S {}

/// Styles separated by comma.
///
/// The macro input will be parsed as [style syntaxes](style::one) separated by comma.
/// Then all styles will be [`chained`](style::chain).
#[macro_export]
macro_rules! style {
    ($($t:tt)*) => {
        $crate::style::syntax::paren!(
            @{$crate::style::syntax}
            ($($t)*)
        )
    };
}

pub mod style;
