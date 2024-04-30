pub use dom_token::{put_tokens_at, DomToken, UniqueDomTokenArray, UniqueDomTokens};

use async_str_iter::AsyncStrIterator;

mod dom_token;

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

    type DomTokensIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter;
}

pub trait ConstPossibleDomTokens {
    const POSSIBLE_DOM_TOKENS: UniqueDomTokens<'static, 'static>;
}

pub mod __private {
    pub use bool;
    pub use str;
    pub use Option;

    pub use async_str_iter::either::IterEither;

    use async_str_iter::IntoAsyncStrIterator;

    pub type IterConcat<T> =
        <async_str_iter::concat::Concat<T> as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    pub fn iter_concat<T>(t: T) -> IterConcat<T>
    where
        async_str_iter::concat::Concat<T>: IntoAsyncStrIterator,
    {
        IntoAsyncStrIterator::into_async_str_iterator(async_str_iter::concat::Concat(t))
    }

    pub type IterStrSlice<'a, 'b> = std::slice::Iter<'a, &'b str>;
    pub fn iter_str_slice<'a, 'b>(s: &'a [&'b str]) -> IterStrSlice<'a, 'b> {
        s.iter()
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

const fn non_empty_count<const N: usize>(tokens: [&'static str; N]) -> usize {
    let mut count = 0;
    let mut i = 0;
    while i < N {
        if !tokens[i].is_empty() {
            count += 1;
        }
        i += 1;
    }

    count
}

#[doc(hidden)]
#[macro_export]
macro_rules! __put_dom_tokens_at {
    ({} $res:ident, $at:ident) => {};
    ({$dom_token:literal} $res:ident, $at:ident) => {{
        $res[$at] = $crate::DomToken::new_const($dom_token);
        $at += 1;
    }};
    ($dom_tokens:tt $res:ident, $at:ident) => {
        ($res, $at) = $crate::put_tokens_at($res, $at, $crate::UniqueDomTokens::as_slice($crate::__unique_dom_tokens! $dom_tokens))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate {
    ($dom_token:literal) => {
        ()
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        ()
    };
    (( $e:expr ) as $as_ty:ty) => {
        $e
    };
    (if ( $e:expr ) $if_block:tt $(else $else_block:tt)?) => {
        $e
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __dom_token_predicate_ty {
    ($dom_token:literal) => {
        ()
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        ()
    };
    (( $e:expr ) as $as_ty:ty) => {
        $crate::__private::bool
    };
    (if ( $e:expr ) $if_block:tt $(else $else_block:tt)?) => {
        $crate::__private::bool
    };
}

#[macro_export]
macro_rules! __anonymous_custom_dom_tokens {
    (
        $($dom_token:tt)+
    ) => {{
        struct AnonymousCustomDomTokens {
            _inner: ($($crate::__dom_token_predicate_ty! $dom_token ,)+)
        }

        const _: () = {
            const POSSIBLE_DOM_TOKENS_COUNT: usize = {
                0
                $(+ $crate::__dom_tokens_count! $dom_token)+
            };

            const POSSIBLE_DOM_TOKEN_ARRAY: $crate::UniqueDomTokenArray<'static, POSSIBLE_DOM_TOKENS_COUNT> = $crate::UniqueDomTokenArray::new_const({
                let mut res = [$crate::DomToken::new_const("_"); POSSIBLE_DOM_TOKENS_COUNT];
                let mut i = 0;
                $($crate::__put_dom_tokens_at!($dom_token res, i);)+
                assert!(i == POSSIBLE_DOM_TOKENS_COUNT);

                res
            });


            impl $crate::ConstPossibleDomTokens for AnonymousCustomDomTokens {
                const POSSIBLE_DOM_TOKENS: $crate::UniqueDomTokens<'static, 'static> = POSSIBLE_DOM_TOKEN_ARRAY.as_unique_dom_tokens();
            }

            impl $crate::DomTokens for AnonymousCustomDomTokens {
                type UpdateWithState = $crate::__private::Option<Self>;

                fn update_with_state(
                    this: Self,
                    dom_token_list: &mut impl $crate::DomTokenList,
                    state: &mut Self::UpdateWithState,
                ) {
                    if let $crate::__private::Option::Some(state) = state {

                    } else {

                    }
                }

                type DomTokensIntoAsyncStrIter = $crate::__private::IterConcat<(
                    $($crate::DomTokensIntoAsyncStrIter! $dom_token,)+
                )>;

                fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
                    $crate::__private::iter_concat((
                        $($crate::dom_tokens_into_async_str_iter! $dom_token,)+
                    ))
                }
            }
        };

        AnonymousCustomDomTokens {
            _inner: ($($crate::__dom_token_predicate! $dom_token ,)+)
        }
    }};
}

#[macro_export]
macro_rules! dom_tokens {
    () => {
        ()
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
    ([$($dom_token:literal),+ $(,)?]) => {
        $crate::__private::IterStrSlice<'static, 'static>
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::DomTokensIntoAsyncStrIter
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__private::Option<$crate::DomTokensIntoAsyncStrIter! $if_block>
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        $crate::__private::IterEither<
            $crate::DomTokensIntoAsyncStrIter! $if_block,
            $crate::DomTokensIntoAsyncStrIter! $else_block,
        >
    };
}

#[macro_export]
macro_rules! dom_tokens_into_async_str_iter {
    ($dom_token:literal) => {
        $dom_token
    };
    ([$($dom_token:literal),+ $(,)?]) => {
        $crate::__private::iter_str_slice(&[$($dom_token),+])
    };
    (( $e:expr ) as $as_ty:ty) => {
        <$as_ty as $crate::DomTokens>::DomTokensIntoAsyncStrIter::dom_tokens_into_async_str_iter($e)
    };
    (if ( $e:expr ) $if_block:tt) => {
        $crate::__private::bool::then($e, || $crate::dom_tokens_into_async_str_iter! $if_block)
    };
    (if ( $e:expr ) $if_block:tt else $else_block:tt) => {
        if $e {
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

#[cfg(test)]
mod tests {
    mod anonymous {
        const _: () = {
            let _: () = dom_tokens!();
            {
                let tokens = dom_tokens!("a");
            }
            {
                let circle = true;
                let array = false;
                let dark = false;
                let tokens = dom_tokens!(
                    "my-btn",
                    if circle {
                        "circle"
                    },
                    if array {
                        ["a", "b"]
                    },
                    if dark { "dark" } else { "light" }
                );
            }
        };
    }
}
