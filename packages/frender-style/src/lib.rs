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

/// An inline expr of [`ConstDeclarationList<impl HasConstDeclarationList>`](crate::constness::ConstDeclarationList).
#[macro_export]
macro_rules! const_style {
    ($s:expr) => {{
        enum HasConstDeclarationList {}
        $crate::impl_has_const_declaration_list_for! {
            impl HasConstDeclarationList {
                const _: _ = $s;
            }
        }
        $crate::constness::ConstDeclarationList::<HasConstDeclarationList>()
    }};
}

#[macro_export]
macro_rules! chain_styles {
    () => {
        $crate::Empty
    };
    ($e:expr $(,)?) => {
        $e
    };
    ($a:expr, $b:expr $(,)?) => {
        $crate::styles::chain::Chain($a, $b)
    };
    ($a:expr, $($rest:expr),+ $(,)? ) => {
        $crate::styles::chain::Chain(
            $a,
            $crate::chain_styles!($($rest),+),
        )
    };
}

/// Styles separated by comma.
///
/// The macro input will be parsed as [style syntaxes](#supported-style-syntaxes) separated by comma.
/// Then all styles will be [`chained`](chain_styles).
///
/// ### Supported style syntaxes
///
/// #### literal and `const {..}`
///
/// They will be parsed as [`const_style`].
///
/// #### block `{..}`
///
/// The block will not be parsed. it will be directly used as an expr so it should implement [`Style`].
///
/// #### array `[..]`
///
/// The array will not be parsed. it will be directly used as an expr so it should implement [`Style`].
///
/// #### parenthesis `(..)`
///
/// The content will be parsed with [`style`].
///
/// #### if else
///
/// - `if ($predicate) { $one_style }` will be parsed as `Option<impl Style>`
/// - `if ($predicate) { $one_style } else { $one_style }` will be parsed as `Either<impl Style, impl Style>`
///
/// Note that the predicate must be wrapped in `( )`.
///
/// #### match
///
///
/// Note that the matched expr must be wrapped in `( )`.
#[macro_export]
macro_rules! style {
    ($s:literal $(, $($rest:tt)*)?) => {
        $crate::chain_styles!(
            $crate::const_style!($s)
            $(, $crate::style!($($rest)*))?
        )
    };
    (const $s:block $(, $($rest:tt)*)?) => {
        $crate::chain_styles!(
            $crate::const_style!($s)
            $(, $crate::style!($($rest)*))?
        )
    };
    ($e:block $(, $($rest:tt)*)?) => {
        $crate::chain_styles!(
            $e
            $(, $crate::style!($($rest)*))?
        )
    };
}

#[macro_export]
macro_rules! one_style {
    ($s:literal) => {
        $crate::const_style!($s)
    };
    (const $s:block) => {
        $crate::const_style!($s)
    };
    ($e:block) => {
        $e
    };
    ([$($array:tt)*]) => {
        [$($array)*]
    };
    (($($style:tt)*)) => {
        $crate::style!($($style)*)
    };
    (if ($predicate:expr) $if_block:tt) => {
        if $predicate {
            $crate::__private::Some(
                $crate::style! $if_block
            )
        } else {
            $crate::__private::None
        }
    };
    (if ($predicate:expr) $if_block:tt else $($after_else:tt)*) => {
        if $predicate {
            $crate::styles::either::EitherStyle::A(
                $crate::style! $if_block
            )
        } else {
            $crate::styles::either::EitherStyle::B(
                $crate::__one_style_after_else! {
                    $($after_else)*
                }
            )
        }
    };
    (match ($matched:expr) {}) => {
        match $matched {}
    };
    (match ($matched:expr) {
        // TODO:
    }) => {
        match $matched {

        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __one_style_after_else {
    ($else_block:tt) => {
        $crate::style! $else_block
    };
    ($($else_if:tt)+) => {
        $crate::style! {
            $($else_if)+
        }
    };
}

#[doc(hidden)]
pub mod __private {
    pub use {None, Some};
}
