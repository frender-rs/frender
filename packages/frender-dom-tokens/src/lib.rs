pub use self::either::EitherDomTokens;
pub use self::erase_const_known::EraseConstKnownPossibleDomTokens;
pub use chain::Chain;
use constness::HasConstKnownPossibleDomTokens;
pub use dom_token::{DomToken, UniqueDomTokenArray, UniqueDomTokenArrayVec, UniqueDomTokens};
pub use frender_common::Empty;

use async_str_iter::AsyncStrIterator;

mod chain;
mod dom_token;
mod either;
mod empty;
mod erase_const_known;
mod option;
mod string;

#[cfg(feature = "web")]
mod web;

/// See [DOMTokenList](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList).
pub trait DomTokenList {
    fn set_value(&mut self, value: &str);
    fn add_1(&mut self, token: DomToken);
    fn remove_1(&mut self, token: DomToken);
    fn replace(&mut self, old_token: DomToken, new_token: DomToken);
}

pub trait DomTokens {
    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::UpdateWithState,
    );

    fn remove_with_state(dom_token_list: &mut impl DomTokenList, state: &mut Self::UpdateWithState);

    type DomTokensIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter;

    fn erase_const_known_possible_dom_tokens(self) -> EraseConstKnownPossibleDomTokens<Self>
    where
        Self: Sized,
    {
        EraseConstKnownPossibleDomTokens(self)
    }
}

pub mod ssr {
    pub mod asserts {
        use async_str_iter::AsyncStrIterator;

        mod sealed {
            pub trait DomTokensPrefixSpace {}
        }

        /// [Empty](async_str_iter::empty::Empty) or multiple space separated dom tokens prefixed with a space.
        pub trait DomTokensPrefixSpace: AsyncStrIterator + sealed::DomTokensPrefixSpace {}

        // empty
        impl sealed::DomTokensPrefixSpace for async_str_iter::empty::Empty {}
        impl DomTokensPrefixSpace for async_str_iter::empty::Empty {}

        // never
        impl sealed::DomTokensPrefixSpace for async_str_iter::never::Never {}
        impl DomTokensPrefixSpace for async_str_iter::never::Never {}

        // option
        impl<T: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
            for async_str_iter::option::IterOption<T>
        {
        }
        impl<T: DomTokensPrefixSpace> DomTokensPrefixSpace for async_str_iter::option::IterOption<T> {}

        // either
        impl<L: DomTokensPrefixSpace, R: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
            for async_str_iter::either::IterEither<L, R>
        {
        }
        impl<L: DomTokensPrefixSpace, R: DomTokensPrefixSpace> DomTokensPrefixSpace
            for async_str_iter::either::IterEither<L, R>
        {
        }

        // chain
        impl<A: DomTokensPrefixSpace, B: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
            for async_str_iter::chain::Chain<A, B>
        {
        }
        impl<A: DomTokensPrefixSpace, B: DomTokensPrefixSpace> DomTokensPrefixSpace
            for async_str_iter::chain::Chain<A, B>
        {
        }

        // const
        impl<T: ?Sized + crate::constness::HasConstDomTokens> sealed::DomTokensPrefixSpace
            for crate::constness::ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
        {
        }
        impl<T: ?Sized + crate::constness::HasConstDomTokens> DomTokensPrefixSpace
            for crate::constness::ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
        {
        }
    }
}

mod sealed {
    pub trait ChainableDomTokens {}

    // empty
    impl ChainableDomTokens for crate::Empty {}

    // const
    impl<T: ?Sized + crate::constness::HasConstDomTokens> ChainableDomTokens
        for crate::constness::ConstDomTokens<T>
    {
    }

    // option
    impl<T: ChainableDomTokens> ChainableDomTokens for Option<T> {}

    // either
    impl<A: ChainableDomTokens, B: ChainableDomTokens> ChainableDomTokens
        for crate::EitherDomTokens<A, B>
    {
    }

    // chain
    impl<A: ChainableDomTokens, B: ChainableDomTokens> ChainableDomTokens for crate::Chain<A, B> {}

    // EraseConstKnownPossibleDomTokens
    impl<T: ChainableDomTokens> ChainableDomTokens for crate::EraseConstKnownPossibleDomTokens<T> {}

    // IntoDomTokens
    impl<T: crate::IntoDomTokens> ChainableDomTokens for T where T::IntoDomTokens: ChainableDomTokens {}
}

/// This trait is sealed to make sure <code>
/// [ChainableDomTokens::DomTokensPrefixSpaceIntoAsyncStrIter] == " " + [DomTokens::DomTokensIntoAsyncStrIter]
/// </code>
pub trait ChainableDomTokens:
    DomTokens + sealed::ChainableDomTokens + HasConstKnownPossibleDomTokens
{
    type DomTokensPrefixSpaceIntoAsyncStrIter: ssr::asserts::DomTokensPrefixSpace;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter;
}

pub trait IntoDomTokens {
    type IntoDomTokens: DomTokens;

    fn into_dom_tokens(self) -> Self::IntoDomTokens;
}

// We expect a value that `impl DomTokens` at const runtime.
// So we can't `impl<T: DomTokens> IntoDomTokens for T`
// because `fn into_dom_tokens` can't be const fn in stable rust.
impl<T: IntoDomTokens> DomTokens for T {
    type UpdateWithState = <T::IntoDomTokens as DomTokens>::UpdateWithState;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        <T::IntoDomTokens>::update_with_state(this.into_dom_tokens(), dom_token_list, state)
    }

    fn remove_with_state(
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        <T::IntoDomTokens>::remove_with_state(dom_token_list, state)
    }

    type DomTokensIntoAsyncStrIter = <T::IntoDomTokens as DomTokens>::DomTokensIntoAsyncStrIter;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        <T::IntoDomTokens>::dom_tokens_into_async_str_iter(this.into_dom_tokens())
    }
}

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

impl<T: IntoDomTokens> ChainableDomTokens for T
where
    T::IntoDomTokens: ChainableDomTokens,
{
    type DomTokensPrefixSpaceIntoAsyncStrIter =
        <T::IntoDomTokens as ChainableDomTokens>::DomTokensPrefixSpaceIntoAsyncStrIter;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        <T::IntoDomTokens as ChainableDomTokens>::dom_tokens_prefix_space_into_async_str_iter(
            this.into_dom_tokens(),
        )
    }
}

pub mod constness;

pub mod dom_tokens {
    pub use frender_const_expr::{array_len, const_marker};

    #[macro_export]
    macro_rules! dom_tokens {
        ($($t:tt)*) => {
            $crate::dom_tokens::syntax::paren!(
                @{$crate::dom_tokens::syntax}
                ($($t)*)
            )
        };
    }

    pub use dom_tokens as comma_separated;

    #[doc(hidden)]
    #[macro_export]
    macro_rules! dom_tokens_one {
        ($($t:tt)*) => {
            $crate::dom_tokens::syntax::one!(
                @{$crate::dom_tokens::syntax}
                $($t)*
            )
        };
    }

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

    pub mod syntax {
        pub use frender_const_expr::syntax::*;

        pub use crate::{Chain, EitherDomTokens as Either, Empty};

        pub use super::r#const;

        pub mod macros {
            pub use dom_tokens;
        }
    }

    pub mod typed {
        pub use frender_const_expr::{
            assert_expr, expand_parsed, resolve_either_paths, syntax as default_syntax,
        };

        pub mod const_markers {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_const_markers_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::const_markers::syntax::one! (
                        @{$crate::dom_tokens::typed::const_markers::syntax}
                        $($t)*
                    );
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_const_markers_one as one;

            pub mod syntax {
                pub use frender_const_expr::syntax::*;

                pub use super::super::common_syntax::{chain, r#macro};

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_empty {
                    (@$with:tt) => {};
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
                macro_rules! dom_tokens_typed_const_markers_syntax_const {
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

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_const_markers_syntax_if {
                    (
                        @{$($with:tt)*}
                        $(#$attr:tt)*
                        if ($($predicate:tt)*) $if_block:tt
                    ) => {
                        pub(crate) mod r#if {
                            $($with)*::block! {
                                @{$($with)*}
                                $(#$attr)*
                                $if_block
                            }
                        }
                    };
                    (
                        @{$($with:tt)*}
                        $(#$attr:tt)*
                        if ($($predicate:tt)*) $if_block:tt
                        else $($after_else:tt)*
                    ) => {
                        pub(crate) mod r#if {
                            $($with)*::block! {
                                @{$($with)*}
                                $(#$attr)*
                                $if_block
                            }
                        }

                        pub(crate) mod r#else {
                            $($with)*::one! {
                                @{$($with)*}
                                $(#$attr)*
                                $($after_else)*
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
                        $crate::dom_tokens::typed::expand_parsed! {
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
                    dom_tokens_typed_const_markers_syntax_const as r#const,
                    dom_tokens_typed_const_markers_syntax_empty as empty,
                    dom_tokens_typed_const_markers_syntax_if as r#if,
                    dom_tokens_typed_const_markers_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_const_markers_syntax_noop as noop,
                };

                pub use {noop as native_block, noop as never};
            }
        }

        pub mod r#type {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_type_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::r#type::syntax::one! {
                        @{$crate::dom_tokens::typed::r#type::syntax} $($t)*
                    }
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_type_one as one;

            pub mod syntax {
                pub use crate::{EitherDomTokens as Either, Empty};
                pub use Option;

                pub use frender_const_expr::syntax::*;

                pub use super::super::common_syntax::{chain, empty, r#macro};

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_chain_impl {
                    ($($chain:tt)*) => {
                        $crate::Chain::<$($chain)*>
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_native_block {
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        $e:block as $as_ty:ty
                    ) => {
                        $(#$attr)*
                        $as_ty
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_const {
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

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_type_syntax_if {
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        if ($($predicate:tt)*) $if_block:tt
                    ) => {
                        $($with)*::Option::<
                            $($with)*::block! {
                                @{$($with)*}
                                #[__dom_tokens_typed_path($($p)*::r#if)]
                                $(#$attr)*
                                $if_block
                            }
                        >
                    };
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        if ($($predicate:tt)*) $if_block:tt
                        else $($after_else:tt)*
                    ) => {
                        $($with)*::Either::<
                            $($with)*::block! {
                                @{$($with)*}
                                #[__dom_tokens_typed_path($($p)*::r#if)]
                                $(#$attr)*
                                $if_block
                            },
                            $($with)*::one!(
                                @{$($with)*}
                                #[__dom_tokens_typed_path($($p)*::r#else)]
                                $(#$attr)*
                                $($after_else)*
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
                        $crate::dom_tokens::typed::expand_parsed! {
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
                        $crate::EitherDomTokens::<
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
                    dom_tokens_typed_type_syntax_const as r#const,
                    dom_tokens_typed_type_syntax_if as r#if,
                    dom_tokens_typed_type_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_type_syntax_native_block as native_block,
                    dom_tokens_typed_type_syntax_never as never,
                };
            }
        }

        pub mod expr {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_expr_one {
                ($($t:tt)*) => {
                    $crate::dom_tokens::typed::expr::syntax::one! {
                        @{$crate::dom_tokens::typed::expr::syntax} $($t)*
                    }
                };
            }

            #[doc(inline)]
            pub use dom_tokens_typed_expr_one as one;

            pub mod syntax {
                pub use frender_const_expr::syntax::*;

                pub use crate::{EitherDomTokens as Either, Empty};

                pub use super::super::common_syntax::{chain, empty, r#macro};

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_chain_impl {
                    ($($chain:tt)*) => {
                        $crate::Chain($($chain)*)
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_native_block {
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        $e:block $(as $as_ty:ty)?
                    ) => {
                        $(#$attr)*
                        $e
                    };
                }

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_const {
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

                #[doc(hidden)]
                #[macro_export]
                macro_rules! dom_tokens_typed_expr_syntax_if {
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        $if:ident $paren_predicate:tt $if_block:tt
                    ) => {
                        $if $crate::dom_tokens::typed::assert_expr!$paren_predicate {
                            $($with)*::Some(
                                $($with)*::block! {
                                    @{$($with)*}
                                    #[__dom_tokens_typed_path($($p)*::r#if)]
                                    $(#$attr)*
                                    $if_block
                                }
                            )
                        } else {
                            $($with)*::None
                        }
                    };
                    (
                        @{$($with:tt)*}
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $(#$attr:tt)*
                        $if:ident $paren_predicate:tt $if_block:tt
                        $else:ident $($after_else:tt)*
                    ) => {
                        $if $crate::dom_tokens::typed::assert_expr!$paren_predicate {
                            $($with)*::Either::A(
                                $($with)*::block!(
                                    @{$($with)*}
                                    #[__dom_tokens_typed_path($($p)*::r#if)]
                                    $(#$attr)*
                                    $if_block
                                )
                            )
                        } $else {
                            $($with)*::Either::B(
                                $($with)*::one!(
                                    @{$($with)*}
                                    #[__dom_tokens_typed_path($($p)*::r#else)]
                                    $(#$attr)*
                                    $($after_else)*
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
                        $crate::dom_tokens::typed::expand_parsed! {
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
                        #[__dom_tokens_typed_path($($p:tt)*)]
                        $e:expr
                    ) => {
                        $crate::dom_tokens::typed::default_syntax::never! {
                            @{$($with)*}
                            $e
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
                        $match $crate::dom_tokens::typed::assert_expr!$paren_matched {
                            $(
                                $($pat)* => $crate::dom_tokens::typed::resolve_either_paths!(
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
                    dom_tokens_typed_expr_syntax_const as r#const,
                    dom_tokens_typed_expr_syntax_if as r#if,
                    dom_tokens_typed_expr_syntax_match_non_empty as match_non_empty,
                    dom_tokens_typed_expr_syntax_native_block as native_block,
                    dom_tokens_typed_expr_syntax_never as never,
                };
            }
        }

        mod common_syntax {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_common_syntax_empty {
                (
                    @{$($with:tt)*}
                    #[__dom_tokens_typed_path($($p:tt)*)]
                ) => {
                    $($with)*::Empty
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
                    parsed {$kind:ident $bang:tt ($($parsed:tt)*)}
                    rest {$($rest:tt)*}
                ) => {
                    $($with)*::chain_impl! {
                        $($with)* :: $kind $bang {
                            @{$($with)*}
                            #[__dom_tokens_typed_path($($p)*::chain0)]
                            $($($attrs)*)?
                            $($parsed)*
                        },
                        $($with)* :: paren! {
                            @{$($with)*}
                            #[__dom_tokens_typed_path($($p)*::chain1)]
                            $($($attrs)*)?
                            ($($rest)*)
                        }
                    }
                };
                (
                    with {$($with:tt)*}
                    attrs {
                        $($attrs:tt)*
                    }
                    parsed {$kind:ident $bang:tt ($($parsed:tt)*)}
                    rest {$($rest:tt)*}
                ) => {
                    $($with)*::chain_impl! {
                        $($with)* :: $kind $bang {
                            @{$($with)*}
                            $($($attrs)*)?
                            $($parsed)*
                        },
                        $($with)* :: paren! {
                            @{$($with)*}
                            $($($attrs)*)?
                            ($($rest)*)
                        }
                    }
                };
            }

            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_common_syntax_macro {
                (@{$($with:tt)*} $(#$attr:tt)* $dom_tokens:ident $bang:tt [$($macro_content:tt)*]) => {
                    $crate::dom_tokens_typed_common_syntax_macro! {
                        @{$($with)*} $(#$attr)* $dom_tokens $bang ($($macro_content)*)
                    }
                };
                (@{$($with:tt)*} $(#$attr:tt)* $dom_tokens:ident $bang:tt {$($macro_content:tt)*}) => {
                    $crate::dom_tokens_typed_common_syntax_macro! {
                        @{$($with)*} $(#$attr)* $dom_tokens $bang ($($macro_content)*)
                    }
                };
                (@{$($with:tt)*} $(#$attr:tt)* $dom_tokens:ident $bang:tt $macro_content:tt) => {
                    $crate::dom_tokens::typed::macros::$dom_tokens $bang {
                        $($with)*::paren! {
                            @{$($with)*}
                            $(#$attr)*
                            $macro_content
                        }
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

        pub mod macros {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! dom_tokens_typed_macros_dom_tokens {
                ($($t:tt)*) => {
                    $($t)*
                };
            }

            pub use dom_tokens_typed_macros_dom_tokens as dom_tokens;
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
