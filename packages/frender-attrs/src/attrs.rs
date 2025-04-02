//! See [`attrs!`](crate::attrs!).

/// Attrs separated by comma.
///
/// The macro input will be parsed as [attrs syntaxes](one) separated by comma.
/// Then all attrs will be [chained](crate::values::Chain).
#[macro_export]
macro_rules! attrs {
    ($($t:tt)*) => {
        $crate::attrs::syntax::paren!(
            @{$crate::attrs::syntax}
            {($($t)*)}
        )
    };
}

#[doc(no_inline)]
pub use attrs as comma_separated;

/// An inline expr of [`ConstAttributes<impl HasConstAttributes>`](type@crate::values::r#const::ConstAttributes).
#[doc(hidden)]
#[macro_export]
macro_rules! attrs_const {
    ($($t:tt)*) => {
        $crate::attrs::__private::r#const! {
            #[const_impl_mod($crate::attrs::__private::const_impl)]
            $($t)*
        }
    };
}

/// ### Supported attrs syntaxes
///
/// #### literal and `const {..}`
///
/// They will be parsed as a [`const attrs`](const!).
///
/// #### verbatim expr `verbatim!(..)`
///
/// The content will not be parsed. it will be directly used as an expr.
/// It should implement [`IntoAttributes`].
///
/// #### block `{..}`
///
/// The block content will be parsed as [`one attrs`](one).
///
/// #### array `[..]`
///
/// The array will not be parsed. it will be directly used as an expr.
/// The array item should implement [`IntoAttributes`]
/// so that the array implements [`IntoAttributes`].
///
/// #### parenthesis `(..)`
///
/// The content will be parsed with [`attrs`].
///
/// #### if else
///
/// - `if ($predicate:expr) { $s:one_attrs }` will be parsed as `if $predicate { Some(one!({ $s })) } else { None }`
/// - `if ($predicate:expr) { $s:one_attrs } else { $t:one_attrs }` will be parsed as `if $predicate { EitherAttributes::A(one!({ $s })) } else { EitherAttributes::B(one!({ $t })) }`
/// - `if ($predicate:expr) { $one_attrs } else if ($predicate) { $one_attrs } ..` will be parsed recursively.
///
/// Note that the predicate must be wrapped in `( )`.
///
/// #### match
///
/// - `match (never) {}` will be parsed as a value of [`Never`](crate::values::Never)
/// - `match ($expr) { _ => $s:one_attrs }` will be parsed as `match $expr { _ => one!($s) }`
/// - `match ($expr) { _ => $s:one_attrs, _ => $t:one_attrs }` will be parsed as `match $expr { _ => EitherAttributes::A(one!($s)), _ => EitherAttributes::B(one!($t)) }`
///
/// Note that the matched expr must be wrapped in `( )`.
///
/// [`IntoAttributes`]: crate::IntoAttributes
#[doc(hidden)]
#[macro_export]
macro_rules! attrs_one {
    ($($t:tt)+) => {
        $crate::attrs::syntax::one!(
            @{$crate::attrs::syntax}
            {$($t)+}
        )
    };
}

#[doc(inline)]
pub use {attrs_const as r#const, attrs_one as one};

pub mod syntax {
    pub use frender_const_expr::syntax::*;

    pub mod parsed {
        pub use crate::values::{Chain, EitherAttributes as Either, Empty, Never};

        pub use super::super::r#const;
    }

    pub mod macros {
        #[doc(no_inline)]
        pub use frender_const_expr::syntax::macros::verbatim;

        #[doc(no_inline)]
        pub use attrs;
    }
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use frender_const_expr::r#const;

    #[doc(hidden)]
    pub mod const_impl {
        pub use crate::{
            impl_HasConstAttributes_for as impl_marker_for,
            values::r#const::ConstAttributes as ConstValue,
        };
    }
}

#[cfg(test)]
mod tests;
