use constness::HasConstKnownPossibleDomTokens;
pub use dom_token::{DomToken, UniqueDomTokenArray, UniqueDomTokenArrayVec, UniqueDomTokens};

pub mod values;

mod dom_token;

#[cfg(feature = "experimental")]
pub mod experimental;

#[cfg(feature = "csr")]
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

mod sealed;

pub trait DomTokens: sealed::DomTokens {
    fn erase_const_known_possible_dom_tokens(self) -> values::EraseConstKnownPossibleDomTokens<Self>
    where
        Self: Sized,
    {
        values::EraseConstKnownPossibleDomTokens(self)
    }
}

/// This trait is sealed to make sure <code>
/// [ChainableDomTokens::DomTokensPrefixSpaceIntoAsyncStrIter] == " " + [DomTokens::DomTokensIntoAsyncStrIter]
/// </code>
pub trait ChainableDomTokens:
    DomTokens + sealed::ChainableDomTokens + HasConstKnownPossibleDomTokens
{
}

pub trait IntoDomTokens {
    type IntoDomTokens: DomTokens;

    fn into_dom_tokens(self) -> Self::IntoDomTokens;
}

// We expect a value that `impl DomTokens` at const runtime.
// So we can't `impl<T: DomTokens> IntoDomTokens for T`
// because `fn into_dom_tokens` can't be const fn in stable rust.
impl<T: IntoDomTokens> crate::sealed::DomTokens for T {}
impl<T: IntoDomTokens> DomTokens for T {}

impl<T: IntoDomTokens> HasConstKnownPossibleDomTokens for T
where
    T::IntoDomTokens: HasConstKnownPossibleDomTokens,
{
    type KnownPossibleDomTokensArrayVecCap =
        <T::IntoDomTokens as HasConstKnownPossibleDomTokens>::KnownPossibleDomTokensArrayVecCap;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC:
        <Self::KnownPossibleDomTokensArrayVecCap as constness::IsConstUsize>::UniqueDomTokenArrayVec<
            'static,
        > =
        <T::IntoDomTokens as HasConstKnownPossibleDomTokens>::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC;
}

impl<T: IntoDomTokens> crate::sealed::ChainableDomTokens for T where
    T::IntoDomTokens: ChainableDomTokens
{
}
impl<T: IntoDomTokens> ChainableDomTokens for T where T::IntoDomTokens: ChainableDomTokens {}

pub mod constness;

pub mod dom_tokens {
    pub use frender_const_expr::{array_len, const_marker};

    #[macro_export]
    macro_rules! dom_tokens {
        ($($t:tt)*) => {
            $crate::dom_tokens::syntax::paren!(
                @{$crate::dom_tokens::syntax}
                {($($t)*)}
            )
        };
    }

    #[doc(no_inline)]
    pub use dom_tokens as comma_separated;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! dom_tokens_one {
        ($($t:tt)*) => {
            $crate::dom_tokens::syntax::one!(
                @{$crate::dom_tokens::syntax}
                {$($t)*}
            )
        };
    }

    // TODO: refactor with frender-const-value
    /// An inline expr of [`ConstDomTokens<impl HasConstDomTokens>`](crate::constness::ConstDomTokens).
    #[doc(hidden)]
    #[macro_export]
    macro_rules! dom_tokens_const {
        (const $($rest:tt)*) => {{
            enum HasConstDomTokens {}
            $crate::dom_tokens::r#const! {
                #[const_marker(HasConstDomTokens)]
                const $($rest)*
            }
        }};
        (
            #[$const_marker:ident $const_marker_body:tt]
            const $s:tt
        ) => {
            $crate::dom_tokens::r#const! {
                #[$const_marker $const_marker_body]
                const $s as _
            }
        };
        (
            #[$const_marker:ident $const_marker_body:tt]
            const $s:tt as $($const_ty:tt)*
        ) => {{
            const CONST_EXPR: $crate::constness::ConstDomTokens::<
                $crate::dom_tokens::const_marker::$const_marker!$const_marker_body
            > = {
                $crate::impl_has_const_dom_tokens_for! {
                    impl $crate::dom_tokens::const_marker::$const_marker!$const_marker_body {
                        const _: $crate::__dom_tokens_infer_const_type![$s $($const_ty)*] = $s;
                    }
                }

                $crate::constness::ConstDomTokens()
            };

            CONST_EXPR
        }};
    }

    #[doc(inline)]
    pub use {dom_tokens_const as r#const, dom_tokens_one as one};

    #[doc(hidden)]
    #[macro_export]
    macro_rules! __dom_tokens_infer_const_type {
        ({ [$($array:tt)*] } _) => {
            [$crate::constness::StaticStr; $crate::dom_tokens::array_len!([$($array)*])]
        };
        ({ [$($array:tt)*] } [$item_ty:ty; _]) => {
            [$item_ty; $crate::dom_tokens::array_len!([$($array)*])]
        };
        ($block:tt _) => {
            $crate::constness::StaticStr
        };
        ($block:tt $ty:ty) => {
            $ty
        };
    }

    #[doc(hidden)]
    pub mod syntax {
        #[doc(no_inline)]
        pub use frender_const_expr::syntax::*;

        pub mod parsed {
            #[doc(no_inline)]
            pub use frender_const_expr::syntax::parsed::*;

            #[doc(no_inline)]
            pub use crate::values::{Chain, EitherDomTokens as Either, Empty};
            // TODO: Never

            #[doc(no_inline)]
            pub use super::super::r#const;
        }

        pub mod macros {
            #[doc(no_inline)]
            pub use dom_tokens;

            #[doc(no_inline)]
            pub use frender_const_expr::syntax::macros::verbatim;
        }
    }

    #[doc(hidden)]
    pub mod typed {
        #[doc(hidden)]
        pub mod __private {
            #[doc(no_inline)]
            pub use frender_const_expr::{
                assert_expr, expand_parsed, resolve_either_paths,
                syntax::never as default_syntax_never,
            };
        }

        pub mod const_markers {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_const_markers_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::const_markers::syntax::one! (
                        @{$crate::dom_tokens::typed::const_markers::syntax}
                        {$($t)*}
                    );
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_const_markers_one as one;

            pub mod syntax {
                pub use frender_const_expr::syntax::*;

                pub mod parsed {
                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! dom_tokens_typed_const_markers_syntax_parsed_const {
                        (
                            #[const_marker($($const_marker_ty:tt)*)]
                            const $($rest:tt)*
                        ) => {};
                        (
                            const $($rest:tt)*
                        ) => {
                            pub enum DomTokensAnonymousHasConstDomTokens {}
                        };
                    }

                    #[doc(inline)]
                    pub use dom_tokens_typed_const_markers_syntax_parsed_const as r#const;
                }

                pub use super::super::common_syntax::{chain, r#macro};

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_empty {
                    (@$with:tt #$attrs:tt {()} {()}) => {};
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_chain_impl {
                    (
                        $a:item,
                        $b:item
                    ) => {
                        pub(crate) mod chain0 {
                            $a
                        }
                        pub(crate) mod chain1 {
                            $b
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_if {
                    (
                        @{$($with:tt)*}
                        #$attrs:tt
                        {if ($($predicate:tt)*) $if_block:tt}
                        $_repeat:tt
                    ) => {
                        pub(crate) mod r#if {
                            $($with)*::block! {
                                @{$($with)*}
                                #$attrs
                                {$if_block}
                                {$if_block}
                            }
                        }
                    };
                    (
                        @{$($with:tt)*}
                        #$attrs:tt
                        {
                            if ($($predicate:tt)*) $if_block:tt
                            else $($after_else:tt)*
                        }
                        $_repeat:tt
                    ) => {
                        pub(crate) mod r#if {
                            $($with)*::block! {
                                @{$($with)*}
                                #$attrs
                                {$if_block}
                                {$if_block}
                            }
                        }

                        pub(crate) mod r#else {
                            $($with)*::one! {
                                @{$($with)*}
                                #$attrs
                                {$($after_else)*}
                                {$($after_else)*}
                            }
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! __dom_tokens_typed_const_markers_syntax_parsed_patterns {
                    (
                        {
                            with $with:tt
                            attrs $attrs:tt
                            parsed_pattern {
                                pat $pat:tt
                                either_paths $either_paths:tt
                                parsed $parsed:tt
                            }
                        }
                    ) => {
                        $crate::dom_tokens::typed::__private::expand_parsed! {
                            with $with
                            attrs $attrs
                            parsed $parsed
                        }
                    };
                    (
                        $a:tt
                        $($rest:tt)+
                    ) => {
                        #[allow(non_snake_case)]
                        pub(crate) mod A {
                            $crate::__dom_tokens_typed_const_markers_syntax_parsed_patterns! {
                                $a
                            }
                        }
                        #[allow(non_snake_case)]
                        pub(crate) mod B {
                            $crate::__dom_tokens_typed_const_markers_syntax_parsed_patterns! {
                                $($rest)+
                            }
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_match_non_empty {
                    (
                        with $with:tt
                        attrs $attrs:tt
                        match {$match:ident}
                        paren_matched {$paren_matched:tt}
                        parsed_patterns {
                            $($parsed_pattern:tt)+
                        }
                    ) => {
                        $crate::__dom_tokens_typed_const_markers_syntax_parsed_patterns! {
                            $({
                                with $with
                                attrs $attrs
                                parsed_pattern $parsed_pattern
                            })+
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_noop {
                    ($($t:tt)*) => {};
                }

                #[doc(inline)]
                pub use {
                    dom_tokens_typed_const_markers_syntax_chain_impl as chain_impl,
                    dom_tokens_typed_const_markers_syntax_empty as empty,
                    dom_tokens_typed_const_markers_syntax_if as r#if,
                    dom_tokens_typed_const_markers_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_const_markers_syntax_noop as noop,
                };

                pub use noop as never;

                pub mod macro_imps {
                    pub use super::super::super::macro_imps::dom_tokens;
                    pub use super::noop as verbatim;
                }
            }
        }

        pub mod r#type {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_type_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::r#type::syntax::one! {
                        @{$crate::dom_tokens::typed::r#type::syntax}
                        {$($t)*}
                    }
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_type_one as one;

            pub mod syntax {
                pub use frender_const_expr::syntax::*;

                pub mod parsed {
                    pub use Option;

                    pub use crate::values::{EitherDomTokens as Either, Empty};

                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! dom_tokens_typed_type_syntax_parsed_const {
                        (
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            #[$const_marker:ident ($($const_marker_ty:tt)*)]
                            const $($rest:tt)*
                        ) => {
                            $crate::constness::ConstDomTokens::<
                                $crate::dom_tokens::const_marker::$const_marker![$($const_marker_ty)*]
                            >
                        };
                        (
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            const $($rest:tt)*
                        ) => {
                            $crate::constness::ConstDomTokens::<$($p)*::DomTokensAnonymousHasConstDomTokens>
                        };
                    }

                    #[doc(inline)]
                    pub use dom_tokens_typed_type_syntax_parsed_const as r#const;
                }

                pub use super::super::common_syntax::{chain, empty, r#macro};

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_chain_impl {
                    ($($chain:tt)*) => {
                        $crate::values::Chain::<$($chain)*>
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_if {
                    (
                        @{$($with:tt)*}
                        #{
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attr:tt)*
                        }
                        {if ($($predicate:tt)*) $if_block:tt}
                        $_repeat:tt
                    ) => {
                        $($attr)*
                        $($with)*::parsed::Option::<
                            $($with)*::block! {
                                @{$($with)*}
                                #{
                                    #[__dom_tokens_typed_path($($p)*::r#if)]
                                }
                                {$if_block}
                                {$if_block}
                            }
                        >
                    };
                    (
                        @{$($with:tt)*}
                        #{
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attr:tt)*
                        }
                        {
                            if ($($predicate:tt)*) $if_block:tt
                            else $($after_else:tt)*
                        }
                        $_repeat:tt
                    ) => {
                        $($attr)*
                        $($with)*::parsed::Either::<
                            $($with)*::block! {
                                @{$($with)*}
                                #{
                                    #[__dom_tokens_typed_path($($p)*::r#if)]
                                }
                                {$if_block}
                                {$if_block}
                            },
                            $($with)*::one!(
                                @{$($with)*}
                                #{
                                    #[__dom_tokens_typed_path($($p)*::r#else)]
                                }
                                {$($after_else)*}
                                {$($after_else)*}
                            )
                        >
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! __dom_tokens_typed_type_syntax_parsed_patterns {
                    (
                        {
                            with $with:tt
                            attrs {
                                #[__dom_tokens_typed_path($($p:tt)*)]
                                $($attrs:tt)*
                            }
                            parsed_pattern {
                                pat $pat:tt
                                either_paths {$($either_paths:tt)*}
                                parsed $parsed:tt
                            }
                        }
                    ) => {
                        $crate::dom_tokens::typed::__private::expand_parsed! {
                            with $with
                            attrs {
                                #[__dom_tokens_typed_path($($p)* $(:: $either_paths)*)]
                                $($attrs)*
                            }
                            parsed $parsed
                        }
                    };
                    (
                        $a:tt
                        $($rest:tt)+
                    ) => {
                        $crate::values::EitherDomTokens::<
                            $crate::__dom_tokens_typed_type_syntax_parsed_patterns! {
                                $a
                            },
                            $crate::__dom_tokens_typed_type_syntax_parsed_patterns! {
                                $($rest)+
                            }
                        >
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_never {
                    ($($t:tt)*) => {
                        $crate::Never
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_match_non_empty {
                    (
                        with $with:tt
                        attrs $attrs:tt
                        match {$match:ident}
                        paren_matched {$paren_matched:tt}
                        parsed_patterns {
                            $($parsed_pattern:tt)+
                        }
                    ) => {
                        $crate::__dom_tokens_typed_type_syntax_parsed_patterns! {
                            $({
                                with $with
                                attrs $attrs
                                parsed_pattern $parsed_pattern
                            })+
                        }
                    };
                }

                #[doc(inline)]
                pub use {
                    dom_tokens_typed_type_syntax_chain_impl as chain_impl,
                    dom_tokens_typed_type_syntax_if as r#if,
                    dom_tokens_typed_type_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_type_syntax_never as never,
                };

                pub mod macro_imps {
                    pub use super::super::super::macro_imps::dom_tokens;

                    #[macro_export]
                    #[doc(hidden)]
                    macro_rules! dom_tokens_typed_type_syntax_macro_imps_verbatim {
                        (@{$($with:tt)*} #{$(#$attr:tt)*} $grouped_macro_content:tt as $as_ty:ty) => {
                            $as_ty
                        };
                    }

                    pub use dom_tokens_typed_type_syntax_macro_imps_verbatim as verbatim;
                }
            }
        }

        pub mod expr {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_expr_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::expr::syntax::one! {
                        @{$crate::dom_tokens::typed::expr::syntax}
                        {$($t)*}
                    }
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_expr_one as one;

            pub mod syntax {
                pub use frender_const_expr::syntax::*;

                pub use super::super::common_syntax::{chain, empty, r#macro};

                pub mod parsed {
                    pub use frender_const_expr::syntax::parsed::*;

                    pub use crate::values::{EitherDomTokens as Either, Empty};

                    #[doc(hidden)]
                    #[macro_export]
                    macro_rules! dom_tokens_typed_expr_syntax_parsed_const {
                        (
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            #[const_marker($($const_marker_ty:tt)*)]
                            const $($rest:tt)*
                        ) => {
                            $crate::dom_tokens::r#const! {
                                #[const_marker($($const_marker_ty)*)]
                                const $($rest)*
                            }
                        };
                        (
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            const $($rest:tt)*
                        ) => {
                            $crate::dom_tokens::r#const!(
                                #[const_marker($($p)*::DomTokensAnonymousHasConstDomTokens)]
                                const $($rest)*
                            )
                        };
                    }

                    #[doc(inline)]
                    pub use dom_tokens_typed_expr_syntax_parsed_const as r#const;
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_chain_impl {
                    ($($chain:tt)*) => {
                        $crate::values::Chain($($chain)*)
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_if {
                    (
                        @{$($with:tt)*}
                        #{
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attr:tt)*
                        }
                        {$if:ident $paren_predicate:tt $if_block:tt}
                        $_repeat:tt
                    ) => {
                        $($attr)*
                        $if $crate::dom_tokens::typed::__private::assert_expr!$paren_predicate {
                            $($with)*::parsed::Some(
                                $($with)*::block! {
                                    @{$($with)*}
                                    #{
                                        #[__dom_tokens_typed_path($($p)*::r#if)]
                                    }
                                    {$if_block}
                                    {$if_block}
                                }
                            )
                        } else {
                            $($with)*::parsed::None
                        }
                    };
                    (
                        @{$($with:tt)*}
                        #{
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attr:tt)*
                        }
                        {
                            $if:ident $paren_predicate:tt $if_block:tt
                            $else:ident $($after_else:tt)*
                        }
                        $_repeat:tt
                    ) => {
                        $($attr)*
                        $if $crate::dom_tokens::typed::__private::assert_expr!$paren_predicate {
                            $($with)*::parsed::Either::A(
                                $($with)*::block!(
                                    @{$($with)*}
                                    #{
                                        #[__dom_tokens_typed_path($($p)*::r#if)]
                                    }
                                    {$if_block}
                                    {$if_block}
                                )
                            )
                        } $else {
                            $($with)*::parsed::Either::B(
                                $($with)*::one!(
                                    @{$($with)*}
                                    #{
                                        #[__dom_tokens_typed_path($($p)*::r#else)]
                                    }
                                    {$($after_else)*}
                                    {$($after_else)*}
                                )
                            )
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! __dom_tokens_typed_expr_syntax_expand_parsed_with_either_paths {
                    (
                        with $with:tt
                        attrs {
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attrs:tt)*
                        }
                        either_paths {$($either_paths:tt)*}
                        parsed $parsed:tt
                    ) => {
                        $crate::dom_tokens::typed::__private::expand_parsed! {
                            with $with
                            attrs {
                                #[__dom_tokens_typed_path($($p)* $(::$either_paths)*)]
                                $($attrs)*
                            }
                            parsed $parsed
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_never {
                    (
                        @{$($with:tt)*}
                        #{
                            #[__dom_tokens_typed_path($($p:tt)*)]
                            $($attr:tt)*
                        }
                        $braced:tt
                        $repeat:tt
                    ) => {
                        $crate::dom_tokens::typed::__private::default_syntax_never! {
                            @{$($with)*}
                            #{$($attr)*}
                            $braced
                            $repeat
                        }
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_match_non_empty {
                    (
                        with $with:tt
                        attrs $attrs:tt
                        match {$match:ident}
                        paren_matched {$paren_matched:tt}
                        parsed_patterns {
                            $(
                                {
                                    pat { $($pat:tt)* }
                                    either_paths $either_paths:tt
                                    parsed $parsed:tt
                                }
                            )*
                        }
                    ) => {
                        $match $crate::dom_tokens::typed::__private::assert_expr!$paren_matched {
                            $(
                                $($pat)* => $crate::dom_tokens::typed::__private::resolve_either_paths!(
                                    @$with
                                    $either_paths
                                    {
                                        $crate::__dom_tokens_typed_expr_syntax_expand_parsed_with_either_paths!(
                                            with $with
                                            attrs $attrs
                                            either_paths $either_paths
                                            parsed $parsed
                                        )
                                    }
                                ),
                            )*
                        }
                    };
                }

                #[doc(inline)]
                pub use {
                    dom_tokens_typed_expr_syntax_chain_impl as chain_impl,
                    dom_tokens_typed_expr_syntax_if as r#if,
                    dom_tokens_typed_expr_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_expr_syntax_never as never,
                };

                pub mod macro_imps {
                    pub use super::super::super::macro_imps::dom_tokens;

                    #[macro_export]
                    #[doc(hidden)]
                    macro_rules! dom_tokens_typed_expr_syntax_macro_imps_verbatim {
                        (@{$($with:tt)*} #{$(#$attr:tt)*} $grouped_macro_content:tt as $as_ty:ty) => {
                            $crate::dom_tokens::syntax::macros::verbatim! $grouped_macro_content
                        };
                    }

                    pub use dom_tokens_typed_expr_syntax_macro_imps_verbatim as verbatim;
                }
            }
        }

        mod common_syntax {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_common_syntax_empty {
                (
                    @{$($with:tt)*}
                    #{
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $($attr:tt)*
                    }
                    {()}
                    {()}
                ) => {
                    $($attr)*
                    $($with)*::parsed::Empty
                };
            }

            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_common_syntax_chain {
                (
                    with {$($with:tt)*}
                    attrs {
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $($attrs:tt)*
                    }
                    parsed $parsed:tt
                    rest {$($rest:tt)*}
                ) => {
                    $($attrs)*
                    $($with)*::chain_impl! {
                        $crate::dom_tokens::typed::__private::expand_parsed! {
                            with {$($with)*}
                            attrs {
                                #[__dom_tokens_typed_path($($p)*::chain0)]
                            }
                            parsed $parsed
                        },
                        $($with)* :: paren! {
                            @{$($with)*}
                            #{
                                #[__dom_tokens_typed_path($($p)*::chain1)]
                            }
                            {($($rest)*)}
                            {($($rest)*)}
                        }
                    }
                };
                (
                    with {$($with:tt)*}
                    attrs {
                        $($attrs:tt)*
                    }
                    parsed $parsed:tt
                    rest {$($rest:tt)*}
                ) => {
                    $($attrs)*
                    $($with)*::chain_impl! {
                        $crate::dom_tokens::typed::__private::expand_parsed! {
                            with {$($with)*}
                            attrs {}
                            parsed $parsed
                        },
                        $($with)* :: paren! {
                            @{$($with)*}
                            #{}
                            {($($rest)*)}
                            {($($rest)*)}
                        }
                    }
                };
            }

            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_common_syntax_macro {
                (
                    @{$($with:tt)*} #$attrs:tt
                    {$macro_name:ident $bang:tt $($rest:tt)*}
                    $_repeat:tt
                ) => {
                    $($with)*::macro_imps::$macro_name $bang {
                        @{$($with)*}
                        #$attrs
                        $($rest)*
                    }
                };
            }

            #[doc(inline)]
            pub use {
                dom_tokens_typed_common_syntax_chain as chain,
                dom_tokens_typed_common_syntax_empty as empty,
                dom_tokens_typed_common_syntax_macro as r#macro,
            };
        }

        pub mod macro_imps {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_macro_imps_dom_tokens {
                (@$with:tt #$attrs:tt [$($macro_content:tt)*]) => {
                    $crate::dom_tokens_typed_macro_imps_dom_tokens! {
                        @$with #$attrs ($($macro_content)*)
                    }
                };
                (@$with:tt #$attrs:tt {$($macro_content:tt)*}) => {
                    $crate::dom_tokens_typed_macro_imps_dom_tokens! {
                        @$with #$attrs ($($macro_content)*)
                    }
                };
                (@{$($with:tt)*} #$attrs:tt $paren_macro_content:tt) => {
                    $($with)*::paren! {
                        @{$($with)*}
                        #$attrs
                        {$paren_macro_content}
                        {$paren_macro_content}
                    }
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_macro_imps_dom_tokens as dom_tokens;
        }
    }
}

#[macro_export]
macro_rules! impl_dom_tokens_for {
    (
        |$this:tt: $for_ty:ty| -> $proxy_ty:ty {$($e:tt)*}
    ) => {
        const _: () = {
            impl $crate::IntoDomTokens for $for_ty {
                type IntoDomTokens = $proxy_ty;
                fn into_dom_tokens($this) -> Self::IntoDomTokens {
                    $($e)*
                }
            }
        };
    };
    (
        |$this:tt: $for_ty:ty| $($one_expr_of_dom_tokens:tt)*
    ) => {
        const _: () = {
            mod __dom_tokens_anonymous_const_markers {
                $crate::dom_tokens::typed::const_markers::one! {
                    $($one_expr_of_dom_tokens)*
                }
            }

            $crate::impl_dom_tokens_for! {
                |$this: $for_ty| -> $crate::dom_tokens::typed::r#type::one![
                    #[__dom_tokens_typed_path(__dom_tokens_anonymous_const_markers)]
                    $($one_expr_of_dom_tokens)*
                ] {
                    $crate::dom_tokens::typed::expr::one!(
                        #[__dom_tokens_typed_path(__dom_tokens_anonymous_const_markers)]
                        $($one_expr_of_dom_tokens)*
                    )
                }
            }
        };
    };
}
