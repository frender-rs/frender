//! See [style!].

/// An inline expr of [`ConstDeclarationList<impl HasConstDeclarationList>`](crate::constness::ConstDeclarationList).
#[doc(hidden)]
#[macro_export]
macro_rules! style_const {
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

/// ### Supported style syntaxes
///
/// #### literal and `const {..}`
///
/// They will be parsed as a [`const style`](r#const!).
///
/// #### native block `{{..}}`
///
/// The block will not be parsed. it will be directly used as an expr.
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
            $($t)+
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! style_chain {
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

#[doc(inline)]
pub use {style_chain as chain, style_const as r#const, style_one as one};

pub mod syntax {
    pub use frender_common::const_expr::syntax::*;

    pub use super::{chain, r#const};

    // TODO: EitherA

    #[doc(hidden)]
    #[macro_export]
    macro_rules! style_syntax_never {
        ($e:expr) => {
            (|| -> $crate::styles::Never { $e })()
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! style_syntax_EitherA {
        ($e:expr) => {
            $crate::styles::EitherStyle::A($e)
        };
    }

    #[doc(hidden)]
    #[macro_export]
    macro_rules! style_syntax_EitherB {
        ($e:expr) => {
            $crate::styles::EitherStyle::B($e)
        };
    }

    #[doc(inline)]
    pub use {
        style_syntax_EitherA as EitherA, style_syntax_EitherB as EitherB,
        style_syntax_never as never,
    };
}

#[cfg(test)]
mod tests {
    use super::one;

    #[test]
    fn match_clause() {
        let f = || one!(match (panic!()) {});

        let _ = f as fn() -> crate::styles::Never;

        let crate::Empty = one!(match (()) {
            _ => {}
        });

        let crate::constness::ConstDeclarationList { .. } = one!(match (()) {
            _ => "",
        });

        match one!(match (true) {
            a if a => "",
            _ => {
                {
                    style!()
                }
            }
        }) {
            crate::styles::EitherStyle::A(crate::constness::ConstDeclarationList { .. }) => {}
            crate::styles::EitherStyle::B(crate::Empty) => panic!(),
        }

        match one!(match (1) {
            a if a > 0 => {}
            b if b < 0 => {}
            _ => {}
        }) {
            crate::styles::EitherStyle::A(crate::Empty) => {}
            crate::styles::EitherStyle::B(other) => match other {
                crate::styles::EitherStyle::A(crate::Empty) => panic!(),
                crate::styles::EitherStyle::B(crate::Empty) => panic!(),
            },
        }
    }
}
