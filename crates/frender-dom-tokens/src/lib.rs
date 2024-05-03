pub use chain::Chain;
pub use dom_token::{put_tokens_at, DomToken, UniqueDomTokenArray, UniqueDomTokens};
pub use empty::Empty;

use async_str_iter::AsyncStrIterator;

mod chain;
mod dom_token;
mod either;
mod empty;
mod option;

mod strings_with_predicates;

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

    type DomTokensPrefixSpaceIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter;
}

#[macro_export]
macro_rules! proxy_dom_tokens {
    (|$this:ident| -> $ty:ty { $e:expr }) => {
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

        type DomTokensPrefixSpaceIntoAsyncStrIter =
            <$ty as $crate::DomTokens>::DomTokensPrefixSpaceIntoAsyncStrIter;

        fn dom_tokens_prefix_space_into_async_str_iter(
            $this: Self,
        ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
            <$ty as $crate::DomTokens>::dom_tokens_prefix_space_into_async_str_iter($e)
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
macro_rules! __unique_dom_tokens {
    () => {
        $crate::UniqueDomTokens::EMPTY
    };
    ($dom_token:literal) => {
        $crate::UniqueDomTokens::new_const(&[$crate::DomToken::new_const($dom_token)])
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        $crate::UniqueDomTokens::new_const(&[$( $crate::DomToken::new_const($dom_token) ),+])
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__unique_dom_tokens! $if_block
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::UniqueDomTokenArray::as_unique_dom_tokens(&{
            const A: $crate::UniqueDomTokens<'static, 'static> = $crate::__unique_dom_tokens! $if_block  ;
            const B: $crate::UniqueDomTokens<'static, 'static> = $crate::__unique_dom_tokens! $else_block;

            const N: usize = $crate::UniqueDomTokens::join_count(A, B);

            $crate::UniqueDomTokens::join::<N>(A, B)
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_tokens_count {
    () => {
        0
    };
    ($dom_token:literal) => {
        1
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        [$($dom_token),+].len()
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::ConstPossibleDomTokens>::POSSIBLE_DOM_TOKENS.len()
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__dom_tokens_count! $if_block
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::UniqueDomTokens::join_count(
            $crate::__unique_dom_tokens! $if_block,
            $crate::__unique_dom_tokens! $else_block,
        )
    };
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

// TODO: remove
#[doc(hidden)]
#[macro_export]
macro_rules! __with_unique_ident {
    ($m:ident $bang:tt ( $t0:tt )) => {
        $crate::$m $bang { Field0 $t0 }
    };
    ($m:ident $bang:tt ( $t0:tt $t1:tt )) => {
        $crate::$m $bang { Field1 $t1 }
    };
}

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

// TODO: remove
#[macro_export]
macro_rules! concat_with_space {
    ($dom_token:expr $(,)?) => {
        $dom_token
    };
    ($dom_token:expr $(, $dom_tokens:expr)* $(,)?) => {
        $crate::__private::concat!(
            $dom_token
            $(, " ", $dom_tokens)*
        )
    };
}

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
}

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

#[macro_export]
macro_rules! ConcatDomTokensIntoAsyncStrIter {
    ($dom_token:tt) => {
        $crate::DomTokensIntoAsyncStrIter! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        $crate::__private::IterConcat<(
            $crate::DomTokensIntoAsyncStrIter! $dom_token,
            $(
                $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_tokens,
            )+
        )>
    };
}

#[macro_export]
macro_rules! NestedDomTokensIntoAsyncStrIter {
    ($dom_token:tt) => {
        $crate::DomTokensIntoAsyncStrIter! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        $crate::__private::IterConcat<(
            $crate::DomTokensIntoAsyncStrIter! $dom_token,
            $crate::NestedDomTokensIntoAsyncStrIter![ $($dom_tokens)+ ]
        )>
    };
}

#[macro_export]
macro_rules! concat_dom_tokens_into_async_str_iter {
    ($dom_token:tt) => {
        $crate::dom_tokens_into_async_str_iter! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        $crate::__private::iter_concat((
            $crate::dom_tokens_into_async_str_iter! $dom_token,
            $(
                $crate::dom_tokens_prefix_space_into_async_str_iter! $dom_tokens,
            )+
        ))
    };
}

#[macro_export]
macro_rules! __nested_dom_tokens_into_async_str_iter_override_expr {
    ([$dom_token:tt] {$($prefix_stmts:tt)*} $override:tt) => {{
        $($prefix_stmts)*
        $crate::__dom_tokens_override_expr!($dom_token $dom_token dom_tokens_into_async_str_iter! $override)
    }};
    ([$dom_token:tt $($dom_tokens:tt)+] {$($prefix_stmts:tt)*} $override:tt) => {
        $crate::__private::iter_concat({
            $($prefix_stmts)*
            (
                $crate::__nested_dom_tokens_into_async_str_iter_override_expr!([$dom_token] {} $override),
                $crate::__nested_dom_tokens_prefix_space_into_async_str_iter_override_expr!([$($dom_tokens)+] {$($prefix_stmts)*} $override ),
            )
        })
    };
}

#[macro_export]
macro_rules! __nested_dom_tokens_prefix_space_into_async_str_iter_override_expr {
    ([$dom_token:tt] {$($prefix_stmts:tt)*} $override:tt) => {{
        $($prefix_stmts)*
        $crate::__dom_tokens_override_expr!($dom_token $dom_token dom_tokens_prefix_space_into_async_str_iter! $override)
    }};
    ([$dom_token:tt $($dom_tokens:tt)+] {$($prefix_stmts:tt)*} $override:tt ) => {
        $crate::__private::iter_concat({
            $($prefix_stmts)*
            (
                $crate::__dom_tokens_override_expr!($dom_token $dom_token dom_tokens_prefix_space_into_async_str_iter! $override),
                $crate::__nested_dom_tokens_prefix_space_into_async_str_iter_override_expr!([$($dom_tokens)+] {$($prefix_stmts)*} $override),
            )
        })
    };
}

#[macro_export]
macro_rules! __dom_tokens_override_expr {
    (
        {( $($_e:tt)* ) as     $_as_ty:ty}
        {$e:tt          $as:tt $as_ty:ty }
        $m:ident $bang:tt $override:tt
    ) => {
        $crate::$m $bang { $override $as $as_ty }
    };
    (
        { if        ( $($_e:tt)* ) $($_rest:tt)*}
        { $if:ident $e:tt          $($rest:tt )*}
        $m:ident $bang:tt $override:tt
    ) => {
        $crate::$m $bang { $if $override $($rest )* }
    };
    ($dom_token:tt $tee:tt $m:ident $bang:tt $override:tt) => {
        $crate::$m $bang $dom_token
    };
}

#[macro_export]
macro_rules! concat_dom_tokens_prefix_space_into_async_str_iter {
    ($dom_token:tt) => {
        $crate::dom_tokens_prefix_space_into_async_str_iter! $dom_token
    };
    ($($dom_tokens:tt)+) => {
        $crate::__private::iter_concat((
            $(
                $crate::dom_tokens_prefix_space_into_async_str_iter! $dom_tokens,
            )+
        ))
    };
}

#[macro_export]
macro_rules! ConcatDomTokensPrefixSpaceIntoAsyncStrIter {
    ($dom_token:tt) => {
        $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        $crate::__private::IterConcat<(
            $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_token,
            $(
                $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_tokens,
            )+
        )>
    };
}

#[macro_export]
macro_rules! NestedDomTokensPrefixSpaceIntoAsyncStrIter {
    ($dom_token:tt) => {
        $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        $crate::__private::IterConcat<(
            $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $dom_token,
            $crate::NestedDomTokensPrefixSpaceIntoAsyncStrIter![$($dom_tokens)+]
        )>
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __update_with_state {
    ({$dom_token:literal} $dom_token_list:ident $state:ident) => {
        // whether initialized
        if !*$state {
            $dom_token_list.add_1($dom_token);
            *$state = true;
        }
    };
    ({[$($dom_token:literal),+ $(,)?]} $dom_token_list:ident $state:ident) => {
        // whether initialized
        if !*$state {
            $($dom_token_list.add_1($dom_token);)+
            *$state = true;
        }
    };
    ({( $e:expr ) as $as_ty:ty} $dom_token_list:ident $state:ident) => {
        <$as_ty as $crate::DomTokens>::update_with_state($e, $dom_token_list, $state)
    };
    ({if ( $($e:tt)+ ) $if_block:tt} $dom_token_list:ident $state:ident) => {
        if $($e)+ {
            $crate::__update_with_state!($if_block $dom_token_list $state);
        } else {
            $crate::__remove_with_state!($if_block $dom_token_list $state);
        }
    };
    ({if ( $($e:tt)+ ) $if_block:tt $(else $else_block:tt)?} $dom_token_list:ident $state:ident) => {
        if $($e)+ {
            $crate::__remove_with_state!($else_block $dom_token_list $state);
            $crate::__update_with_state!($if_block   $dom_token_list $state);
        } else {
            $crate::__remove_with_state!($if_block   $dom_token_list $state);
            $crate::__update_with_state!($else_block $dom_token_list $state);
        }
    };
}

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

        AnonymousCustomDomTokens {
            _inner: $crate::__nested_dom_token_predicate!($($dom_token)+)
        }
    }};
}

#[macro_export]
macro_rules! dom_tokens {
    () => {
        $crate::Empty
    };
    ($($t:tt)*) => {
        $crate::__parse_dom_tokens!([]{$($t)*}{$($t)*})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __parse_dom_tokens {
    ($t:tt {}{}) => {
        $crate::__anonymous_custom_dom_tokens! $t
    };
    (
        [$($t:tt)*]
        {$dom_token:literal $(, $($rest:tt)*)?}
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $dom_token }]
            {$($($rest)*)?}
            {$($($rest)*)?}
        )
    };
    ($t:tt { if $_pred:tt $($_rest:tt)* } { $if:ident $pred:tt $($rest:tt)* }) => {
        $crate::__parse_dom_tokens!($t if { $if } ($pred) {$($_rest)*} {$($rest)*})
    };
    ($t:tt if {$($if:tt)*} $pred:tt {{$($_block:tt)*} $($_rest:tt)*} {$block:tt $($rest:tt)*}) => {
        $crate::__parse_dom_tokens!($t if_end { $($if)* $pred $block } {$($_rest)*} {$($rest)*})
    };
    (
        [$($t:tt)*]
        if_end $if:tt
        {$(, $($_rest:tt)*)?}
        {$(, $($rest:tt )*)?}
    ) => {
        $crate::__parse_dom_tokens!([$($t)* $if] { $($($_rest)*)? } { $($($rest)*)? })
    };
    (
        [$($t:tt)*]
        if_end {$($if:tt)*}
        { else        {$($_else_block:tt)*} $(, $($_rest:tt)*)? }
        { $else:tt $else_block:tt        $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $($if)* $else  $else_block }]
            { $($($_rest)*)? }
            { $($($rest)*)? }
        )
    };
    (
        $t:tt
        if_end {$($if:tt)*}
        { else     if          $_pred:tt $($_rest:tt)* }
        { $else:tt $else_if:tt $pred:tt  $($rest:tt)*  }
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            if {$($if:tt)* $else $else_if} ($pred)
            { $($($_rest)*)? }
            { $($($rest)*)? }
        )
    };
    (
        [$($t:tt)*]
        { [$($_array:tt)*] $(, $($_rest:tt)*)? }
        { $array:tt        $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $array }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
        )
    };
    (
        $t:tt
        { $e:tt $($rest:tt)* }
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($e)
            {$($rest)*}
            {$($rest)*}
        )
    };
    (
        [$($t:tt)*]
        expr_as $e:tt
        { as     $_as_ty:ty $(, $($_rest:tt)*)? }
        { $as:tt $as_ty:ty  $(, $($rest:tt )*)? }
    ) => {
        $crate::__parse_dom_tokens!(
            [$($t)* { $e $as $as_ty }]
            {$($($_rest)*)?}
            {$($($rest )*)?}
        )
    };
    (
        $t:tt
        expr_as ($($pre:tt)*)
        { $e:tt $($rest:tt)* }
        $tee:tt
    ) => {
        $crate::__parse_dom_tokens!(
            $t
            expr_as ($($pre)* $e)
            {$($rest)*}
            {$($rest)*}
        )
    };
}

#[macro_export]
macro_rules! DomTokensIntoAsyncStrIter {
    ($dom_token:literal) => {
        &'static $crate::__private::str
    };
    ([$dom_token:literal $(,)?]) => {
        &'static $crate::__private::str
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        &'static [&'static $crate::__private::str]
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::DomTokensIntoAsyncStrIter
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__private::IterOption<$crate::DomTokensIntoAsyncStrIter! $if_block>
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::__private::IterEither<
            $crate::DomTokensIntoAsyncStrIter! $if_block,
            $crate::DomTokensIntoAsyncStrIter! $else_block,
        >
    };
}

#[macro_export]
macro_rules! DomTokensPrefixSpaceIntoAsyncStrIter {
    ($dom_token:literal) => {
        &'static $crate::__private::str
    };
    ([$dom_token:literal $(,)?]) => {
        &'static $crate::__private::str
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        &'static [&'static str]
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::DomTokensPrefixSpaceIntoAsyncStrIter
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__private::IterOption<$crate::DomTokensPrefixSpaceIntoAsyncStrIter! $if_block>
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::__private::IterEither<
            $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $if_block,
            $crate::DomTokensPrefixSpaceIntoAsyncStrIter! $else_block,
        >
    };
}

#[macro_export]
macro_rules! dom_tokens_into_async_str_iter {
    ($dom_token:literal) => {
        $dom_token
    };
    ([$dom_token:literal $(,)?]) => {
        $dom_token
    };
    ([$dom_token:literal $(, $dom_tokens:literal)+ $(,)?]) => {
        <[_; _]>::as_slice(&[$dom_token $(, $crate::__private::concat!(" ", $dom_tokens))+])
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::dom_tokens_into_async_str_iter($e)
    };
    (if ( $($e:tt)+ ) $if_block:tt) => {
        $crate::__private::bool::then($($e)+, || $crate::dom_tokens_into_async_str_iter! $if_block)
    };
    (if ( $($e:tt)+ ) $if_block:tt else $else_block:tt) => {
        if $($e)+ {
            $crate::__private::IterEither::Left(
                $crate::dom_tokens_into_async_str_iter! $if_block
            )
        } else {
            $crate::__private::IterEither::Right(
                $crate::dom_tokens_into_async_str_iter! $else_block
            )
        }
    };
}

#[macro_export]
macro_rules! dom_tokens_prefix_space_into_async_str_iter {
    ($dom_token:literal) => {
        $crate::__private::concat!(" ", $dom_token)
    };
    ([$dom_token:literal $(,)?]) => {
        $crate::__private::concat!(" ", $dom_token)
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        [$($crate::__private::concat!(" ", $dom_token)),+].as_slice()
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::dom_tokens_prefix_space_into_async_str_iter($e)
    };
    (if ( $($e:tt)+ ) $if_block:tt) => {
        $crate::__private::bool::then($($e)+, || $crate::dom_tokens_prefix_space_into_async_str_iter! $if_block)
    };
    (if ( $($e:tt)+ ) $if_block:tt else $else_block:tt) => {
        if $($e)+ {
            $crate::__private::IterEither::Left(
                $crate::dom_tokens_prefix_space_into_async_str_iter! $if_block
            )
        } else {
            $crate::__private::IterEither::Right(
                $crate::dom_tokens_prefix_space_into_async_str_iter! $else_block
            )
        }
    };
}

#[macro_export]
macro_rules! UpdateWithState {
    ($dom_token:literal) => {
        $crate::__private::bool
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        $crate::__private::bool
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::UpdateWithState
    };
    (if ( $($e:tt)+ ) $if_block:tt) => {
        $crate::UpdateWithState! $if_block
    };
    (if ( $($e:tt)+ ) $if_block:tt else $else_block:tt) => {
        $crate::__private::EitherState<
            $crate::UpdateWithState! $if_block,
            $crate::UpdateWithState! $else_block,
        >
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __NestedUpdateWithState {
    ($dom_token:tt) => {
        $crate::UpdateWithState! $dom_token
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        (
            $crate::UpdateWithState! $dom_token,
            $crate::__NestedUpdateWithState![$($dom_tokens)+]
        )
    };
}

#[macro_export]
macro_rules! remove_with_state {
    ({$dom_token:literal} $dom_token_list:ident $dom_token_state:ident) => {
        if *$dom_token_state {
            *$dom_token_state = false;
            $crate::DomTokenList::remove_1($dom_token);
        }
    };
    ({[$($dom_token:literal),+ $(,)?]} $dom_token_list:ident $dom_token_state:ident) => {
        if *$dom_token_state {
            *$dom_token_state = false;
            $(
                $crate::DomTokenList::remove_1($dom_token);
            )+
        }
    };
    ({( $e:expr ) as $as_ty:ty} $dom_token_list:ident $dom_token_state:ident) => {
        <$as_ty as $crate::DomTokens>::remove_with_state($dom_token_list, $dom_token_state)
    };
    ({if ( $($e:tt)+ ) $if_block:tt} $dom_token_list:ident $dom_token_state:ident) => {
        $crate::remove_with_state! { $if_block $dom_token_list $dom_token_state }
    };
    ({if ( $($e:tt)+ ) $if_block:tt else $else_block:tt} $dom_token_list:ident $dom_token_state:ident) => {
        $crate::remove_with_state! { $if_block $dom_token_list $dom_token_state }

        $crate::__private::EitherState<
            $crate::UpdateWithState! $if_block,
            $crate::UpdateWithState! $else_block,
        >
    };
}

#[macro_export]
macro_rules! __nested_remove_with_state {
    ([$dom_token:tt] $dom_token_list:ident $dom_tokens_state:ident) => {
        $crate::remove_with_state! { $dom_token $dom_token_list $dom_tokens_state }
    };
    ([$dom_token:tt $($dom_tokens:tt)+] $dom_token_list:ident $dom_tokens_state:ident) => {
        let (__dom_tokens_state, __dom_tokens_state_rest) = $dom_tokens_state;
        $crate::remove_with_state! { $dom_token $dom_token_list __dom_tokens_state }
        $crate::__nested_remove_with_state! { [$($dom_tokens)+] $dom_token_list __dom_tokens_state_rest }
    };
}

#[cfg(test)]
mod tests {
    mod anonymous {
        use async_str_iter::ext::AsyncStrIteratorExt as _;

        use crate::DomTokens;

        const _: () = {
            {
                let tokens = dom_tokens!("a");
            }
            {
                let circle = true;
                let array = false;
                let dark = false;
            }
        };
    }
}
