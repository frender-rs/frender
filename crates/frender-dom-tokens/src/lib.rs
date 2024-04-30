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

    type DomTokensPrefixSpaceIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter;
}

pub trait ConstPossibleDomTokens {
    const POSSIBLE_DOM_TOKENS: UniqueDomTokens<'static, 'static>;
}

pub mod __private {
    pub use bool;
    pub use str;
    pub use Option;

    pub use core::concat;

    pub use async_str_iter::{either::IterEither, option::IterOption};

    use async_str_iter::IntoAsyncStrIterator;

    pub type IterConcat<T> =
        <async_str_iter::concat::Concat<T> as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    pub fn iter_concat<T>(t: T) -> IterConcat<T>
    where
        async_str_iter::concat::Concat<T>: IntoAsyncStrIterator,
    {
        IntoAsyncStrIterator::into_async_str_iterator(async_str_iter::concat::Concat(t))
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
    (if ( $($e:tt)+ ) $if_block:tt $(else $else_block:tt)?) => {
        $($e)+
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

#[doc(hidden)]
#[macro_export]
macro_rules! __concat_dom_token_predicate_ty {
    ($dom_token:tt) => {
        ($crate::__dom_token_predicate_ty! $dom_token, ())
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        ($crate::__dom_token_predicate_ty! $dom_token, $crate::__concat_dom_token_predicate_ty![$($dom_tokens)+])
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concat_dom_token_predicate {
    ($dom_token:tt) => {
        ($crate::__dom_token_predicate! $dom_token, ())
    };
    ($dom_token:tt $($dom_tokens:tt)+) => {
        ($crate::__dom_token_predicate! $dom_token, $crate::__concat_dom_token_predicate![$($dom_tokens)+])
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
macro_rules! __define_dom_tokens_type {
    ({$dom_token:literal} $vis:vis type $name:ident) => {
        $vis struct $name;
    };
    ({[$($dom_token:literal),+ $(,)?]}) => {
        ()
    };
    ({( $e:expr ) as $as_ty:ty}) => {
        $crate::__private::bool
    };
    ({if ( $e:expr ) $if_block:tt $(else $else_block:tt)?}) => {
        $crate::__private::bool
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

#[macro_export]
macro_rules! __anonymous_custom_dom_tokens {
    (
        $($dom_token:tt)+
    ) => {{
        #[derive(Debug, Clone, Copy)] // TODO: import from $crate
        struct AnonymousCustomDomTokens {
            _inner: $crate::__concat_dom_token_predicate_ty![$($dom_token)+]
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

                type DomTokensIntoAsyncStrIter = $crate::NestedDomTokensIntoAsyncStrIter![$($dom_token)+];

                fn dom_tokens_into_async_str_iter(Self {
                    _inner: __dom_tokens_rest
                }: Self) -> Self::DomTokensIntoAsyncStrIter {
                    $crate::__nested_dom_tokens_into_async_str_iter_override_expr!(
                        [$($dom_token)+]
                        { let (__dom_tokens_value, __dom_tokens_rest) = __dom_tokens_rest; }
                        (__dom_tokens_value)
                    )
                }

                type DomTokensPrefixSpaceIntoAsyncStrIter = $crate::NestedDomTokensPrefixSpaceIntoAsyncStrIter![$($dom_token)+];

                fn dom_tokens_prefix_space_into_async_str_iter(Self {
                    _inner: __dom_tokens_rest
                }: Self) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
                    $crate::__nested_dom_tokens_into_async_str_iter_override_expr!(
                        [$($dom_token)+]
                        { let (__dom_tokens_value, __dom_tokens_rest) = __dom_tokens_rest; }
                        (__dom_tokens_value)
                    )
                }
            }
        };

        AnonymousCustomDomTokens {
            _inner: $crate::__concat_dom_token_predicate!($($dom_token)+)
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

#[cfg(test)]
mod tests {
    mod anonymous {
        use async_str_iter::ext::AsyncStrIteratorExt as _;

        use crate::DomTokens;

        const _: () = {
            let _: () = dom_tokens!();
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
