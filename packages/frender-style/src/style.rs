//! See [`style!`](crate::style!).

/// Styles separated by comma.
///
/// The macro input will be parsed as [style syntaxes](one) separated by comma.
/// Then all styles will be [chained](crate::styles::Chain).
#[macro_export]
macro_rules! style {
    ($($t:tt)*) => {
        $crate::style::syntax::paren!(
            @{$crate::style::syntax}
            {($($t)*)}
        )
    };
}

#[doc(no_inline)]
pub use style as comma_separated;

/// An inline expr of [`ConstDeclarationList<impl HasConstDeclarationList>`](type@crate::styles::constness::ConstDeclarationList).
#[doc(hidden)]
#[macro_export]
macro_rules! style_const {
    ($($t:tt)*) => {
        $crate::style::__private::r#const! {
            #[const_impl_mod($crate::style::__private::const_impl)]
            $($t)*
        }
    };
}

/// ### Supported style syntaxes
///
/// #### literal and `const {..}`
///
/// They will be parsed as a [`const style`](const!).
///
/// #### verbatim expr `verbatim!(..)`
///
/// The content will not be parsed. it will be directly used as an expr.
/// If used as style attribute value, it should implement [`Style`].
///
/// #### block `{..}`
///
/// The block content will be parsed as [`one style`](one).
///
/// #### array `[..]`
///
/// The array will not be parsed. it will be directly used as an expr.
/// If used as an style attribute value, the array item should implement [`Style`]
/// so that the array implements [`Style`].
///
/// #### parenthesis `(..)`
///
/// The content will be parsed with [`style`].
///
/// #### if else
///
/// - `if ($predicate:expr) { $s:one_style }` will be parsed as `if $predicate { Some(one!({ $s })) } else { None }`
/// - `if ($predicate:expr) { $s:one_style } else { $t:one_style }` will be parsed as `if $predicate { EitherStyle::A(one!({ $s })) } else { EitherStyle::B(one!({ $t })) }`
/// - `if ($predicate:expr) { $one_style } else if ($predicate) { $one_style } ..` will be parsed recursively.
///
/// Note that the predicate must be wrapped in `( )`.
///
/// #### match
///
/// - `match (never) {}` will be parsed as a value of [`Never`](crate::styles::Never)
/// - `match ($expr) { _ => $s:one_style }` will be parsed as `match $expr { _ => one!($s) }`
/// - `match ($expr) { _ => $s:one_style, _ => $t:one_style }` will be parsed as `match $expr { _ => EitherStyle::A(one!($s)), _ => EitherStyle::B(one!($t)) }`
///
/// Note that the matched expr must be wrapped in `( )`.
///
/// [`Style`]: crate::Style
#[doc(hidden)]
#[macro_export]
macro_rules! style_one {
    ($($t:tt)+) => {
        $crate::style::syntax::one!(
            @{$crate::style::syntax}
            {$($t)+}
        )
    };
}

#[doc(inline)]
pub use {style_const as r#const, style_one as one};

pub mod syntax {
    pub use frender_const_expr::syntax::*;

    pub mod parsed {
        pub use crate::styles::{Chain, EitherStyle as Either, Empty, Never};

        pub use super::super::r#const;
    }

    pub mod macros {
        #[doc(no_inline)]
        pub use frender_const_expr::syntax::macros::verbatim;

        #[doc(no_inline)]
        pub use style;
    }
}

#[doc(hidden)]
pub mod __private {
    #[doc(hidden)]
    pub use frender_const_expr::{const_marker, r#const};

    #[doc(hidden)]
    pub mod const_impl {
        #[doc(hidden)]
        pub use crate::{
            impl_has_const_declaration_list_for as impl_marker_for,
            styles::constness::{
                ConstDeclarationList as ConstValue, HasConstDeclarationList as HasConstValue,
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        declaration::Declaration,
        styles::{
            constness::{ConstDeclarationList, HasConstDeclarationList},
            Empty,
        },
    };

    use super::one;

    fn declaration_list_of_expr<T: ?Sized + HasConstDeclarationList>(
        _: &ConstDeclarationList<T>,
    ) -> Vec<(
        T::DeclarationNameStr,
        T::DeclarationValueStr,
        T::DeclarationImportant,
    )> {
        T::DECLARATION_LIST
            .into_iter()
            .map(
                |Declaration {
                     name,
                     value,
                     important,
                 }| { (name.into_unparsed(), value.into_unparsed(), important) },
            )
            .collect()
    }

    #[test]
    fn match_clause() {
        let f = || one!(match (panic!()) {});

        let _ = f as fn() -> crate::styles::Never;

        let Empty = one!(match (()) {
            _ => (),
        });

        let crate::styles::constness::ConstDeclarationList { .. } = one!(match (()) {
            _ => "",
        });

        match one!(match (true) {
            a if a => "",
            _ => style!(),
        }) {
            crate::styles::EitherStyle::A(crate::styles::constness::ConstDeclarationList {
                ..
            }) => {}
            crate::styles::EitherStyle::B(Empty) => unreachable!(),
        }

        match one!(match (1) {
            // empty (think unit tuple `()` as an empty list)
            a if a > 0 => (),
            b if b < 0 => {
                // empty wrapped in a block
                ()
            }
            // empty wrapped in parenthesis
            _ => (()),
        }) {
            crate::styles::EitherStyle::A(Empty) => {}
            crate::styles::EitherStyle::B(other) => match other {
                crate::styles::EitherStyle::A(Empty) => panic!(),
                crate::styles::EitherStyle::B(Empty) => panic!(),
            },
        }
    }

    #[test]
    fn array() {
        assert_eq!(declaration_list_of_expr(&one!([])), vec![]);
        assert_eq!(declaration_list_of_expr(&one!([""])), vec![]);
        assert_eq!(
            declaration_list_of_expr(&one!(["a:b;c:d", ""])),
            vec![("a", "b", Empty), ("c", "d", Empty)]
        );
        assert_eq!(
            declaration_list_of_expr(&one!(["a:b;c:d!important;", ";  e :  f"])),
            vec![("a", "b", false), ("c", "d", true), ("e", "f", false)]
        );
    }
}
