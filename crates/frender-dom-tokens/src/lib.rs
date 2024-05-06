pub use chain::Chain;
pub use dom_token::{put_tokens_at, separate, DomToken, UniqueDomTokenArray, UniqueDomTokens};
pub use empty::Empty;

use async_str_iter::AsyncStrIterator;

mod chain;
mod dom_token;
mod either;
mod empty;
mod option;
mod string;

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
}

pub trait ChainableDomTokens: DomTokens {
    type DomTokensPrefixSpaceIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter;
}

#[macro_export]
macro_rules! proxy_dom_tokens {
    (|$this:tt| -> $ty:ty { $e:expr }) => {
        type UpdateWithState = <$ty as $crate::DomTokens>::UpdateWithState;

        fn update_with_state(
            $this: Self,
            dom_token_list: &mut impl $crate::DomTokenList,
            state: &mut Self::UpdateWithState,
        ) {
            <$ty as $crate::DomTokens>::update_with_state($e, dom_token_list, state)
        }

        fn remove_with_state(
            dom_token_list: &mut impl $crate::DomTokenList,
            state: &mut Self::UpdateWithState,
        ) {
            <$ty as $crate::DomTokens>::remove_with_state(dom_token_list, state)
        }

        type DomTokensIntoAsyncStrIter = <$ty as $crate::DomTokens>::DomTokensIntoAsyncStrIter;

        fn dom_tokens_into_async_str_iter($this: Self) -> Self::DomTokensIntoAsyncStrIter {
            <$ty as $crate::DomTokens>::dom_tokens_into_async_str_iter($e)
        }
    };
}

#[macro_export]
macro_rules! proxy_chainable_dom_tokens {
    (|$this:tt| -> $ty:ty { $e:expr }) => {
        type DomTokensPrefixSpaceIntoAsyncStrIter =
            <$ty as $crate::ChainableDomTokens>::DomTokensPrefixSpaceIntoAsyncStrIter;

        fn dom_tokens_prefix_space_into_async_str_iter(
            $this: Self,
        ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
            <$ty as $crate::ChainableDomTokens>::dom_tokens_prefix_space_into_async_str_iter($e)
        }
    };
}

// pub trait IntoDomTokens {
//     type IntoDomTokens: DomTokens;
//     fn into_dom_tokens(self) -> Self::IntoDomTokens;
// }

// impl<T: DomTokens> IntoDomTokens for T {
//     type IntoDomTokens = T;

//     fn into_dom_tokens(self) -> Self::IntoDomTokens {
//         self
//     }
// }

pub trait ConstPossibleDomTokens {
    const POSSIBLE_DOM_TOKENS: UniqueDomTokens<'static, 'static>;
}

#[doc(hidden)]
pub mod __private {
    pub use bool;
    pub use str;
    pub use usize;
    pub use Option;

    pub use core::{
        assert, concat,
        default::Default,
        pin::Pin,
        task::{Context, Poll},
    };

    pub use async_str_iter::{either::IterEither, option::IterOption, AsyncStrIterator};
    pub use frender_common::either::EitherState;

    pub use either::Either;

    use async_str_iter::IntoAsyncStrIterator;

    pub type IterConcat<T> =
        <async_str_iter::concat::Concat<T> as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    pub fn iter_concat<T>(t: T) -> IterConcat<T>
    where
        async_str_iter::concat::Concat<T>: IntoAsyncStrIterator,
    {
        IntoAsyncStrIterator::into_async_str_iterator(async_str_iter::concat::Concat(t))
    }

    pub const fn str_slice_from_first(s: &str) -> &str {
        if let Some((_, s)) = s.as_bytes().split_first() {
            match std::str::from_utf8(s) {
                Ok(s) => s,
                Err(_) => panic!("str_slice_from_first is not utf8"),
            }
        } else {
            panic!("str_slice_from_first invalid")
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate {
    ($dom_token:literal) => {
        __dom_tokens_types::DomTokens
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        __dom_tokens_types::DomTokens
    };
    (( $e:expr ) as $as_ty:ty) => {
        $e
    };
    (if ( $($e:tt)+ ) $if_block:tt) => {
        if $($e)+ {
            $crate::__private::Option::Some({
                use __dom_tokens_types::__dom_tokens_inner_mod as __dom_tokens_types;
                $crate::__dom_token_predicate! $if_block
            })
        } else {
            $crate::__private::Option::None
        }
    };
    (if ( $($e:tt)+ ) $if_block:tt else $else_block:tt) => {
        if $($e)+ {
            $crate::__private::Either::Left({
                use __dom_tokens_types::__dom_tokens_inner_mod_a as __dom_tokens_types;
                $crate::__dom_token_predicate! $if_block
            })
        } else {
            $crate::__private::Either::Right({
                use __dom_tokens_types::__dom_tokens_inner_mod_b as __dom_tokens_types;
                $crate::__dom_token_predicate! $else_block
            })
        }
    };
    (
        match $e:tt $match_body:tt
    ) => {
        $crate::__dom_token_predicate_match! {
            [__dom_tokens_types]
            {}
            ()
            match $e $match_body
        }
    };
    (
        dom_tokens! $dom_tokens:tt
    ) => {
        $crate::__nested_dom_token_predicate! $dom_tokens
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wrap_nested_either {
    (() $e:expr) => {
        $e
    };
    (($var:ident $rest:tt) $e:expr) => {
        $crate::__wrap_nested_either!(
            $rest
            $crate::__private::Either::$var($e)
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate_match {
    (
        $types_path:tt
        $match_body:tt
        $either_variants:tt
        match ( $($e:tt)+ ) {}
    ) => {
        match $($e)+ $match_body
    };
    (
        [$($types_path:tt)*]
        {$($match_body:tt)*}
        $either_variants:tt
        match ( $($e:tt)+ ) {
            $p0:pat => $braced_one_expr_of_dom_token_0:tt
        }
    ) => {
        match $($e)+ {
            $($match_body)*
            $p0 => {
                $crate::__wrap_nested_either!(
                    $either_variants
                    {
                        use $($types_path)* as __dom_tokens_types;
                        $crate::__dom_token_predicate! $braced_one_expr_of_dom_token_0
                    }
                )
            }
        }
    };
    (
        [$($types_path:tt)*]
        {$($match_body:tt)*}
        $either_variants:tt
        match $e:tt {
            $p0:pat => $braced_one_expr_of_dom_token_0:tt
            $($rest:tt)+
        }
    ) => {
        $crate::__dom_token_predicate_match! {
            [$($types_path)*::__dom_tokens_inner_mod_b]
            {
                $($match_body)*
                $p0 => {
                    $crate::__wrap_nested_either!(
                        (Left $either_variants)
                        {
                            use $($types_path)*::__dom_tokens_inner_mod_a as __dom_tokens_types;
                            $crate::__dom_token_predicate! $braced_one_expr_of_dom_token_0
                        }
                    )
                }
            }
            (Right $either_variants)
            match $e {
                $($rest)+
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nested_dom_token_predicate {
    ($dom_token:tt) => {
        $crate::__dom_token_predicate! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        __dom_tokens_types::DomTokens::new(
            {
                use __dom_tokens_types::__dom_tokens_types_first as __dom_tokens_types;
                $crate::__dom_token_predicate! $dom_token
            },
            {
                use __dom_tokens_types::__dom_tokens_types_rest as __dom_tokens_types;
                $crate::__nested_dom_token_predicate![$($dom_tokens)+]
            }
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_one_str {
    ($vis:vis struct $name:ident; $e:expr) => {
        $vis struct $name($crate::__private::bool);

        impl $crate::__private::AsyncStrIterator for $name {
            fn poll_next_str(
                self: $crate::__private::Pin<&mut Self>,
                _: &mut $crate::__private::Context
            ) -> $crate::__private::Poll<$crate::__private::Option<&$crate::__private::str>> {
                let this = self.get_mut();

                $crate::__private::Poll::Ready(if this.0 {
                    $crate::__private::Option::None
                } else {
                    this.0 = true;
                    $crate::__private::Option::Some($e)
                })
            }
        }

    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_pats {
    (
        $t:tt
        {}
        { $on_finish:tt $bang:tt { $($on_finish_rest:tt)* } }
    ) => {
        $crate::$on_finish $bang {
            $t
            $($on_finish_rest)*
        }
    };
    (
        {$($t:tt)*}
        { $p:pat => {$($braced:tt)*} $(,)? $($rest:tt)* }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens! {
            []
            {$($braced)*}
            {$($braced)*}
            {
                #[parse_one_expr_of_dom_tokens]
                __parse_pats_braced_finish! {
                    {
                        $($t)*
                        $p =>
                    }
                    {$($rest)*}
                    $on_finish
                }
            }
        }
    };
    (
        {$($t:tt)*}
        { $p:pat => $($dom_tokens_and_rest:tt)+ }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens! {
            []
            {$($dom_tokens_and_rest)+}
            {$($dom_tokens_and_rest)+}
            {
                #[parse_one_expr_of_dom_tokens]
                __parse_pats_one_finish! {
                    {
                        $($t)*
                        $p =>
                    }
                    $on_finish
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_pats_braced_finish {
    (
        $braced_one_expr_of_dom_token:tt
        {} // rest after one_expr_of_dom_tokens
        {$($t:tt)*}
        $rest:tt
        $on_finish:tt
    ) => {
        $crate::__parse_pats! {
            { $($t)* $braced_one_expr_of_dom_token }
            $rest
            $on_finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_pats_one_finish {
    (
        $braced_one_expr_of_dom_token:tt
        $rest:tt
        {$($t:tt)*}
        $on_finish:tt
    ) => {
        $crate::__parse_pats! {
            { $($t)* $braced_one_expr_of_dom_token }
            $rest
            $on_finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_dom_tokens_types {
    ({$dom_token:literal} $vis:vis) => {
        $crate::__define_dom_tokens_types! {{[$dom_token]} $vis}
    };
    ({[$($dom_token:literal),+ $(,)?]} $vis:vis) => {
        #[derive(Debug, Clone, Copy)]
        $vis struct DomTokens;

        const DOM_TOKENS_PREFIX_SPACE_INTO_ASYNC_STR_ITER: &$crate::__private::str = $crate::__private::concat!($(" ", $dom_token),+);
        const DOM_TOKENS_INTO_ASYNC_STR_ITER: &$crate::__private::str = $crate::__private::str_slice_from_first(DOM_TOKENS_PREFIX_SPACE_INTO_ASYNC_STR_ITER);

        $crate::__define_one_str! {
            $vis struct DomTokensIntoAsyncStrIter;
            DOM_TOKENS_INTO_ASYNC_STR_ITER
        }

        $crate::__define_one_str! {
            $vis struct DomTokensPrefixSpaceIntoAsyncStrIter;
            DOM_TOKENS_PREFIX_SPACE_INTO_ASYNC_STR_ITER
        }

        impl $crate::DomTokens for DomTokens {
            type UpdateWithState = $crate::__private::bool;

            fn update_with_state(
                Self: Self,
                dom_token_list: &mut impl $crate::DomTokenList,
                state: &mut Self::UpdateWithState,
            ) {
                if !*state {
                    *state = true;
                    $({
                        const DOM_TOKEN: $crate::DomToken<'static> = $crate::DomToken::new_const($dom_token);
                        $crate::DomTokenList::add_1(dom_token_list, DOM_TOKEN);
                    })+
                }
            }

            fn remove_with_state(dom_token_list: &mut impl $crate::DomTokenList, state: &mut Self::UpdateWithState) {
                if *state {
                    *state = false;
                    $({
                        const DOM_TOKEN: $crate::DomToken<'static> = $crate::DomToken::new_const($dom_token);
                        $crate::DomTokenList::remove_1(dom_token_list, DOM_TOKEN);
                    })+
                }
            }

            type DomTokensIntoAsyncStrIter = DomTokensIntoAsyncStrIter;

            fn dom_tokens_into_async_str_iter(Self: Self) -> Self::DomTokensIntoAsyncStrIter {
                DomTokensIntoAsyncStrIter(false)
            }
        }

        impl $crate::ChainableDomTokens for DomTokens {
            type DomTokensPrefixSpaceIntoAsyncStrIter = DomTokensPrefixSpaceIntoAsyncStrIter;

            fn dom_tokens_prefix_space_into_async_str_iter(
                Self: Self,
            ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
                DomTokensPrefixSpaceIntoAsyncStrIter(false)
            }
        }

        $vis const POSSIBLE_DOM_TOKENS_COUNT: $crate::__private::usize =
            [$($dom_token),+].len()
        ;

        $vis const POSSIBLE_DOM_TOKEN_ARRAY: $crate::UniqueDomTokenArray<'static, POSSIBLE_DOM_TOKENS_COUNT> = {
            $crate::UniqueDomTokenArray::new_const([$($crate::DomToken::new_const($dom_token)),+])
        };
    };
    ({( $e:expr ) as $as_ty:ty} pub(in $($vis:tt)+)) => {
        #[allow(unused_imports)] use $($vis)+::super::*;
        pub(in $($vis)+) type DomTokens = $as_ty;

        const POSSIBLE_DOM_TOKENS: $crate::UniqueDomTokens<'static> = <self::DomTokens as $crate::ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS;
        pub(in $($vis)+) const POSSIBLE_DOM_TOKENS_COUNT: $crate::__private::usize = POSSIBLE_DOM_TOKENS.len();
        pub(in $($vis)+) const POSSIBLE_DOM_TOKEN_ARRAY: $crate::UniqueDomTokenArray<'static, POSSIBLE_DOM_TOKENS_COUNT> = $crate::UniqueDomTokenArray::new_const({
            let mut res = [$crate::DomToken::new_const("_"); POSSIBLE_DOM_TOKENS_COUNT];
            let mut at = 0;
            (res, at) = $crate::put_tokens_at(res, at, $crate::UniqueDomTokens::as_slice(POSSIBLE_DOM_TOKENS));
            $crate::__private::assert!(at == POSSIBLE_DOM_TOKENS_COUNT);

            res
        });
    };
    ({if $if:tt $if_block:tt} pub(in $($vis:tt)+)) => {
        pub(in $($vis)+) mod __dom_tokens_inner_mod {
            #[allow(unused_imports)] use $($vis)+::super::*;
            $crate::__define_dom_tokens_types! { $if_block pub(in $($vis)+::super) }
        }
        pub(in $($vis)+) type DomTokens = $crate::__private::Option<__dom_tokens_inner_mod::DomTokens>;
        pub(in $($vis)+) use __dom_tokens_inner_mod::{POSSIBLE_DOM_TOKENS_COUNT, POSSIBLE_DOM_TOKEN_ARRAY};
    };
    ({if $if:tt $if_block:tt else $else_block:tt} pub(in $($vis:tt)+)) => {
        pub(in $($vis)+) mod __dom_tokens_inner_mod_a {
            #[allow(unused_imports)] use $($vis)+::super::*;
            $crate::__define_dom_tokens_types! { $if_block pub(in $($vis)+::super) }
        }
        pub(in $($vis)+) mod __dom_tokens_inner_mod_b {
            #[allow(unused_imports)] use $($vis)+::super::*;
            $crate::__define_dom_tokens_types! { $else_block pub(in $($vis)+::super) }
        }
        pub(in $($vis)+) type DomTokens = $crate::__private::Either<
            __dom_tokens_inner_mod_a::DomTokens,
            __dom_tokens_inner_mod_b::DomTokens
        >;

        pub(in $($vis)+) const POSSIBLE_DOM_TOKENS_COUNT: $crate::__private::usize = {
            __dom_tokens_inner_mod_a::POSSIBLE_DOM_TOKENS_COUNT + __dom_tokens_inner_mod_b::POSSIBLE_DOM_TOKENS_COUNT
        };
        pub(in $($vis)+) const POSSIBLE_DOM_TOKEN_ARRAY: $crate::UniqueDomTokenArray<'static, POSSIBLE_DOM_TOKENS_COUNT> = {
            $crate::UniqueDomTokenArray::new_const({
                let mut res = [$crate::DomToken::new_const("_"); POSSIBLE_DOM_TOKENS_COUNT];
                let mut at = 0;
                (res, at) = $crate::put_tokens_at(res, at, $crate::UniqueDomTokenArray::as_slice(&__dom_tokens_inner_mod_a::POSSIBLE_DOM_TOKEN_ARRAY));
                (res, at) = $crate::put_tokens_at(res, at, $crate::UniqueDomTokenArray::as_slice(&__dom_tokens_inner_mod_b::POSSIBLE_DOM_TOKEN_ARRAY));
                $crate::__private::assert!(at == POSSIBLE_DOM_TOKENS_COUNT);

                res
            })
        };
    };
    (
        {
            match $match:tt {
                $p:pat => $braced_one_expr_of_dom_token:tt
            }
        }
        pub(in $($vis:tt)+)
    ) => {
        $crate::__define_dom_tokens_types! {
            $braced_one_expr_of_dom_token
            pub(in $($vis)+)
        }
    };
    (
        {
            match $match:tt {
                $p0:pat => $braced_one_expr_of_dom_token:tt
                $($rest:tt)+
            }
        }
        pub(in $($vis:tt)+)
    ) => {
        $crate::__define_dom_tokens_types! {
            {
                if () $braced_one_expr_of_dom_token
                else {
                    match () {
                        $($rest)+
                    }
                }
            }
            pub(in $($vis)+)
        }
    };
    (
        {
            dom_tokens ! $dom_tokens:tt
        }
        pub(in $($vis:tt)+)
    ) => {
        $crate::__nested_dom_tokens_types! {
            $dom_tokens
            ($($vis)+)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __nested_dom_tokens_types {
    ([$dom_token:tt] ($($root_path:tt)+)) => {
        $crate::__define_dom_tokens_types! { $dom_token pub(in $($root_path)+) }
    };
    ([$dom_token:tt $($dom_tokens:tt)+] ($($root_path:tt)+)) => {
        pub(in $($root_path)+) mod __dom_tokens_types_first {
            $crate::__define_dom_tokens_types! { $dom_token pub(in $($root_path)+::super) }
        }
        pub(in $($root_path)+) mod __dom_tokens_types_rest {
            $crate::__nested_dom_tokens_types! { [$($dom_tokens)+] ($($root_path)+::super) }
        }

        pub(in $($root_path)+) type DomTokens = $crate::Chain<
            __dom_tokens_types_first::DomTokens,
            __dom_tokens_types_rest::DomTokens,
        >;

        pub(in $($root_path)+) const POSSIBLE_DOM_TOKENS_COUNT: $crate::__private::usize = {
            __dom_tokens_types_first::POSSIBLE_DOM_TOKENS_COUNT + __dom_tokens_types_rest::POSSIBLE_DOM_TOKENS_COUNT
        };
        pub(in $($root_path)+) const POSSIBLE_DOM_TOKEN_ARRAY: $crate::UniqueDomTokenArray<'static, POSSIBLE_DOM_TOKENS_COUNT> = {
            $crate::UniqueDomTokenArray::new_const({
                let mut res = [$crate::DomToken::new_const("_"); POSSIBLE_DOM_TOKENS_COUNT];
                let mut at = 0;
                (res, at) = $crate::put_tokens_at(res, at, $crate::UniqueDomTokenArray::as_slice(&__dom_tokens_types_first::POSSIBLE_DOM_TOKEN_ARRAY));
                (res, at) = $crate::put_tokens_at(res, at, $crate::UniqueDomTokenArray::as_slice(&__dom_tokens_types_rest::POSSIBLE_DOM_TOKEN_ARRAY));
                $crate::__private::assert!(at == POSSIBLE_DOM_TOKENS_COUNT);

                res
            })
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __anonymous_custom_dom_tokens {
    (
        $($dom_token:tt)+
    ) => {{
        mod __dom_tokens_types {
            $crate::__nested_dom_tokens_types! { [$($dom_token)+] (super) }
        }

        #[derive(Debug, Clone, Copy)] // TODO: import from $crate
        struct AnonymousCustomDomTokens {
            _inner: __dom_tokens_types::DomTokens
        }

        impl $crate::ConstPossibleDomTokens for AnonymousCustomDomTokens {
            const POSSIBLE_DOM_TOKENS: $crate::UniqueDomTokens<'static, 'static> = __dom_tokens_types::POSSIBLE_DOM_TOKEN_ARRAY.as_unique_dom_tokens();
        }
        impl $crate::DomTokens for AnonymousCustomDomTokens {
            $crate::proxy_dom_tokens!(|this| -> __dom_tokens_types::DomTokens { this._inner });
        }
        impl $crate::ChainableDomTokens for AnonymousCustomDomTokens {
            $crate::proxy_chainable_dom_tokens!(|this| -> __dom_tokens_types::DomTokens { this._inner });
        }

        AnonymousCustomDomTokens {
            _inner: $crate::__nested_dom_token_predicate!($($dom_token)+)
        }
    }};
}

#[macro_export]
macro_rules! dom_tokens {
    (@$on_finish:tt ($($t:tt)*)) => {
        $crate::dom_tokens! { @$on_finish {$($t)*} }
    };
    (@$on_finish:tt [$($t:tt)*]) => {
        $crate::dom_tokens! { @$on_finish {$($t)*} }
    };
    (@$on_finish:tt $t:tt) => {
        $crate::__parse_dom_tokens! { [] $t $t $on_finish }
    };
    () => {
        $crate::Empty
    };
    ($($t:tt)*) => {
        $crate::__parse_dom_tokens!([]{$($t)*}{$($t)*}{})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens_on_finish_braced {
    (
        $dom_token:tt
        {} // rest of one_expr_of_dom_token
        [$($t:tt)*]
        $rest:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens! {
            [$($t)* $dom_token]
            $rest
            $rest
            $on_finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens_on_finish_macro {
    (
        $bracketed_dom_tokens:tt
        { $dom_tokens:tt $bang:tt }
        [$($t:tt)*]
        $rest:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens! {
            [$($t)* { $dom_tokens $bang $bracketed_dom_tokens }]
            $rest
            $rest
            $on_finish
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens_on_finish_pats {
    (
        $pats:tt
        [$($t:tt)*]
        { $match:tt $pred:tt }
        $rest:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [
                $($t)*
                { $match $pred $pats }
            ]
            $rest
            $rest
            $on_finish
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens {
    ($t:tt {}{} {}) => {
        $crate::__anonymous_custom_dom_tokens! $t
    };
    ($t:tt {}{} { $crate_macro_name:ident $bang:tt }) => {
        $crate::$crate_macro_name $bang $t
    };
    ($t:tt {}{} { $crate_macro_name:ident $bang:tt { $($macro_rest:tt)* } }) => {
        $crate::$crate_macro_name $bang { $t $($macro_rest)* }
    };
    (
        [$one_expr_of_dom_tokens:tt]
        $rest:tt $tee:tt
        { #[parse_one_expr_of_dom_tokens] $crate_macro_name:ident $bang:tt { $($macro_rest:tt)* } }
    ) => {
        $crate::$crate_macro_name $bang { $one_expr_of_dom_tokens $rest $($macro_rest)* }
    };
    // { one_expr_of_dom_tokens }
    (
        $t:tt
        { {$($_one_expr_of_dom_tokens:tt)*} $(, $($_rest:tt)*)?}
        { $one_expr_of_dom_tokens:tt        $(, $($rest:tt )*)?}
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            []
            $one_expr_of_dom_tokens
            $one_expr_of_dom_tokens
            {
                #[parse_one_expr_of_dom_tokens]
                __parse_dom_tokens_on_finish_braced! {
                    $t
                    {$($($rest)*)?}
                    $on_finish
                }
            }
        )
    };
    // "literal"
    (
        [$($t:tt)*]
        {$dom_token:literal $(, $($rest:tt)*)?}
        $tee:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $dom_token }]
            {$($($rest)*)?}
            {$($($rest)*)?}
            $on_finish
        )
    };
    // dom_tokens! ..
    (
        $t:tt
        { dom_tokens     !        $_dom_tokens_content:tt $(, $($_rest:tt)*)? }
        { $dom_tokens:tt $bang:tt $dom_tokens_content:tt  $(, $($rest:tt )*)? }
        $on_finish:tt
    ) => {
        $crate::$dom_tokens $bang {
            @{
                __parse_dom_tokens_on_finish_macro! {
                    { $dom_tokens $bang }
                    $t
                    {$($($rest)*)?}
                    $on_finish
                }
            }
            $dom_tokens_content
        }
    };
    // if
    (
        $t:tt
        { if        $_pred:tt $($_rest:tt)* }
        { $if:ident $pred:tt  $($rest:tt )* }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if { $if } ($pred)
            {$($_rest)*}
            {$($rest )*}
            $on_finish
        )
    };
    (
        $t:tt
        if {$($if:tt)*} $pred:tt
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $($rest:tt )*}
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if_end { $($if)* $pred $block }
            {$($_rest)*}
            {$($rest )*}
            $on_finish
        )
    };
    (
        $t:tt
        if $if:tt ($($pred:tt)*)
        {$non_block:tt $($rest:tt)*}
        $tee:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if $if ($($pred)* $non_block)
            {$($rest)*}
            {$($rest)*}
            $on_finish
        )
    };
    (
        [$($t:tt)*]
        if_end $if:tt
        {$(, $($_rest:tt)*)?}
        {$(, $($rest:tt )*)?}
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* $if]
            { $($($_rest)*)? }
            { $($($rest )*)? }
            $on_finish
        )
    };
    (
        [$($t:tt)*]
        if_end {$($if:tt)*}
        { else        {$($_else_block:tt)*} $(, $($_rest:tt)*)? }
        { $else:tt $else_block:tt        $(, $($rest:tt )*)? }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $($if)* $else  $else_block }]
            { $($($_rest)*)? }
            { $($($rest)*)? }
            $on_finish
        )
    };
    (
        $t:tt
        if_end {$($if:tt)*}
        { else     if          $_pred:tt $($_rest:tt)* }
        { $else:tt $else_if:tt $pred:tt  $($rest:tt)*  }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if {$($if:tt)* $else $else_if} ($pred)
            { $($($_rest)*)? }
            { $($($rest)*)? }
            $on_finish
        )
    };
    (
        [$($t:tt)*]
        { [$($_array:tt)*] $(, $($_rest:tt)*)? }
        { $array:tt        $(, $($rest:tt )*)? }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $array }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
            $on_finish
        )
    };
    // match
    (
        $t:tt
        { match     $_pred:tt $($_rest:tt)* }
        { $match:tt $pred:tt  $($rest:tt )* }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            match $match ($pred)
            {$($_rest)*}
            {$($rest )*}
            $on_finish
        )
    };
    (
        $t:tt
        match $match:tt $pred:tt
        {{$($_block:tt)*} $($_rest:tt)*}
        {$block:tt        $($rest:tt )*}
        $on_finish:tt
    ) => {
        $crate::__parse_pats!(
            {}
            $block
            {
                __parse_dom_tokens_on_finish_pats! {
                    $t
                    { $match $pred }
                    {$($rest )*}
                    $on_finish
                }
            }
        )
    };
    (
        $t:tt
        match $match:tt ($($pred:tt)*)
        {$non_block:tt $($rest:tt)*}
        $tee:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            match $match ($($pred)* $non_block)
            {$($rest)*}
            {$($rest)*}
            $on_finish
        )
    };
    // as
    (
        $t:tt
        { $e:tt $($rest:tt)* }
        $tee:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($e)
            {$($rest)*}
            {$($rest)*}
            $on_finish
        )
    };
    (
        [$($t:tt)*]
        expr_as $e:tt
        { as     $_as_ty:ty $(, $($_rest:tt)*)? }
        { $as:tt $as_ty:ty  $(, $($rest:tt )*)? }
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $e $as $as_ty }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
            $on_finish
        )
    };
    (
        $t:tt
        expr_as ($($pre:tt)*)
        { $e:tt $($rest:tt)* }
        $tee:tt
        $on_finish:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($($pre)* $e)
            {$($rest)*}
            {$($rest)*}
            $on_finish
        )
    };
}

#[macro_export]
macro_rules! impl_dom_tokens_for {
    (
        |$this:tt: $for_ty:ty| -> $proxy_ty:ty { $e:expr }
    ) => {
        impl $crate::DomTokens for $for_ty {
            $crate::proxy_dom_tokens! {
                |$this| -> $proxy_ty { $e }
            }
        }

        impl $crate::ChainableDomTokens for $for_ty {
            $crate::proxy_chainable_dom_tokens! {
                |$this| -> $proxy_ty { $e }
            }
        }

        impl $crate::ConstPossibleDomTokens for $for_ty {
            const POSSIBLE_DOM_TOKENS: $crate::UniqueDomTokens<'static, 'static> =
                <$proxy_ty as ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS;
        }
    };
    (
        |$this:tt: $for_ty:ty| $dom_tokens_macro:ident $bang:tt $dom_tokens_macro_content:tt
    ) => {
        const _: () = {
            $crate::__impl_dom_tokens_for_imp! {
                { $dom_tokens_macro $dom_tokens_macro $bang { $this $for_ty }}
                $dom_tokens_macro_content
            }
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_dom_tokens_for_imp {
    ($m:tt ($($t:tt)*)) => {
        $crate::__impl_dom_tokens_for_imp! {$m {$($t)*}}
    };
    ($m:tt [$($t:tt)*]) => {
        $crate::__impl_dom_tokens_for_imp! {$m {$($t)*}}
    };
    ({ dom_tokens $dom_tokens_macro:ident $bang:tt $data:tt } $dom_tokens_macro_content:tt) => {
        $crate::$dom_tokens_macro $bang {
            @{ __impl_dom_tokens_for_imp_finish ! $data }
            $dom_tokens_macro_content
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_dom_tokens_for_imp_finish {
    ($dom_tokens:tt $this:tt $for_ty:ty) => {
        mod __dom_tokens_types {
            $crate::__nested_dom_tokens_types! { $dom_tokens (super) }
        }

        impl $crate::ConstPossibleDomTokens for $for_ty {
            const POSSIBLE_DOM_TOKENS: $crate::UniqueDomTokens<'static, 'static> =
                __dom_tokens_types::POSSIBLE_DOM_TOKEN_ARRAY.as_unique_dom_tokens();
        }

        fn __dom_tokens_get_value($this: $for_ty) -> __dom_tokens_types::DomTokens {
            $crate::__nested_dom_token_predicate! $dom_tokens
        }

        impl $crate::DomTokens for $for_ty {
            $crate::proxy_dom_tokens!(|this| -> __dom_tokens_types::DomTokens {
                __dom_tokens_get_value(this)
            });
        }

        impl $crate::ChainableDomTokens for $for_ty {
            $crate::proxy_chainable_dom_tokens!(|this| -> __dom_tokens_types::DomTokens {
                __dom_tokens_get_value(this)
            });
        }
    };
}
