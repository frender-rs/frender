// mod advanced; // TODO: impl advanced dom tokens

use async_str_iter::AsyncStrIterator;
use frender_common::write::str::AsyncWritableStr;

#[doc(hidden)]
pub mod custom_const_dom_tokens {
    use super::DomTokenList;

    pub fn update_dom_token_list<const N: usize>(
        dom_tokens: [&str; N],
        predicates: [bool; N],
        dom_token_list: &mut impl DomTokenList,
        old_predicates: &mut [bool; N],
    ) {
        for ((token, predicate), state) in
            dom_tokens.into_iter().zip(predicates).zip(old_predicates)
        {
            if predicate != *state {
                if predicate {
                    dom_token_list.add_1(token);
                } else {
                    dom_token_list.remove_1(token);
                }
                *state = predicate;
            }
        }
    }

    pub fn update_dom_token_list_and_initialize_state<const N: usize>(
        dom_tokens: [&str; N],
        predicates: [bool; N],
        dom_token_list: &mut impl DomTokenList,
    ) -> [bool; N] {
        for (token, predicate) in dom_tokens.into_iter().zip(predicates) {
            if predicate {
                dom_token_list.add_1(token);
            }
        }

        predicates
    }
}

#[macro_export]
macro_rules! dom_tokens {
    [
        $($dom_token:tt $(= $predicate:expr)?),*
        $(,)?
    ] => {{
        const DOM_TOKEN_COUNT: usize = $crate::__count_tt!($($dom_token)*);

        #[derive(::core::clone::Clone, ::core::marker::Copy)]
        struct CustomConstDomTokens([bool; DOM_TOKEN_COUNT]);
        impl CustomConstDomTokens {
            const DOM_TOKEN_COUNT: usize = DOM_TOKEN_COUNT;
            const DOM_TOKENS: [&'static str; DOM_TOKEN_COUNT] = [$($dom_token),*];
        }

        // TODO: asserts dom tokens are unique

        impl $crate::dom_token::DomTokens for CustomConstDomTokens {
            type UpdateState = [::core::primitive::bool; Self::DOM_TOKEN_COUNT];

            fn update_dom_token_list_and_initialize_state(
                this: Self,
                dom_token_list: &impl $crate::dom_token::DomTokenList,
            ) -> Self::UpdateState {
                $crate::dom_token::custom_const_dom_tokens::update_dom_token_list(
                    Self::DOM_TOKENS,
                    this.0,
                    dom_token_list,
                )
            }

            fn update_dom_token_list(
                this: Self,
                dom_token_list: &impl $crate::dom_token::DomTokenList,
                state: &mut Self::UpdateState,
            ) {
                $crate::dom_token::custom_const_dom_tokens::update_dom_token_list(
                    Self::DOM_TOKENS,
                    this.0,
                    dom_token_list,
                    state,
                )
            }

            type StringChunk = &'static ::core::primitive::str;
            type StringChunks = $crate::dom_token::StringChunksWithPredicates<{ Self::DOM_TOKEN_COUNT }>;

            fn into_string_chunks(this: Self) -> Self::StringChunks {
                $crate::dom_token::StringChunksWithPredicates::new(Self::DOM_TOKENS, this.0)
            }
        }

        CustomConstDomTokens([$($crate::__predicate_or_true!($($predicate)?)),*])
    }};
}

pub struct StringChunksWithPredicates<const N: usize> {
    strings: [&'static str; N],
    predicates: [bool; N],
    current: usize,
}

impl<const N: usize> StringChunksWithPredicates<N> {
    pub fn new(strings: [&'static str; N], predicates: [bool; N]) -> Self {
        Self {
            strings,
            predicates,
            current: 0,
        }
    }
}

impl<const N: usize> Iterator for StringChunksWithPredicates<N> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current >= N {
                return None;
            }
            if !self.predicates[self.current] {
                self.current += 1;
            } else {
                self.current += 1;
                return Some(self.strings[self.current]);
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __count_tt {
    () => {
        0
    };
    ($t1:tt) => {
        1
    };
    ($t1:tt $t2:tt) => {
        2
    };
    ($t1:tt $t2:tt $t3:tt) => {
        3
    };
    ($t1:tt $t2:tt $t3:tt $t4:tt) => {
        4
    };
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt) => {
        5
    };
    ($t1:tt $t2:tt $t3:tt $t4:tt $t5:tt $($t:tt)+) => {
        5 + $crate::__count_tt!($($t)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __predicate_or_true {
    () => {
        true
    };
    ($predicate:expr) => {
        $predicate
    };
}

/// See [DOMTokenList](https://developer.mozilla.org/en-US/docs/Web/API/DOMTokenList).
pub trait DomTokenList {
    fn set_value(&mut self, value: &str);
    fn add_1(&mut self, token: &str);
    fn remove_1(&mut self, token: &str);
    fn replace(&mut self, old_token: &str, new_token: &str);
}

pub trait DomTokens {
    type UpdateState;

    fn update_dom_token_list_and_initialize_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
    ) -> Self::UpdateState;

    fn update_dom_token_list(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::UpdateState,
    );

    type UpdateWithState: Default;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl DomTokenList,
        state: &mut Self::UpdateWithState,
    );

    type AsyncWritableDomTokens: AsyncWritableStr;

    fn maybe_into_async_writable_dom_tokens(this: Self) -> Option<Self::AsyncWritableDomTokens>;

    type DomTokensIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_maybe_into_async_str_iter(this: Self) -> Option<Self::DomTokensIntoAsyncStrIter>;
}

mod impl_for_static_string {
    use std::borrow::Cow;

    use async_str_iter::IntoAsyncStrIterator;
    use frender_common::write::str::StrWriting;

    use super::DomTokens;

    crate::impl_many!(
        impl<__> DomTokens
            for each_of![
                &'static str,
                String,
                Cow<'static, str>,
                std::rc::Rc<str>,
                std::sync::Arc<str>,
            ]
        {
            type UpdateState = Self;

            fn update_dom_token_list_and_initialize_state(
                this: Self,
                dom_token_list: &mut impl super::DomTokenList,
            ) -> Self::UpdateState {
                dom_token_list.set_value(&this);
                this
            }

            fn update_dom_token_list(
                this: Self,
                dom_token_list: &mut impl super::DomTokenList,
                state: &mut Self::UpdateState,
            ) {
                if *state != this {
                    dom_token_list.set_value(&this);
                    *state = this
                }
            }

            type UpdateWithState = Option<Self>;

            fn update_with_state(
                this: Self,
                dom_token_list: &mut impl super::DomTokenList,
                state: &mut Self::UpdateWithState,
            ) {
                if let Some(state) = state {
                    if *state == this {
                        return;
                    }
                }

                dom_token_list.set_value(&this);
                *state = Some(this)
            }

            type AsyncWritableDomTokens = StrWriting<Self>;
            fn maybe_into_async_writable_dom_tokens(
                this: Self,
            ) -> Option<Self::AsyncWritableDomTokens> {
                Some(StrWriting::new(this))
            }

            type DomTokensIntoAsyncStrIter = <Self as IntoAsyncStrIterator>::IntoAsyncStrIterator;

            fn dom_tokens_maybe_into_async_str_iter(
                this: Self,
            ) -> Option<Self::DomTokensIntoAsyncStrIter> {
                Some(this.into_async_str_iterator())
            }
        }
    );
}

mod impl_for_unit_tuple {
    use crate::NeverWritable;

    use super::DomTokens;

    impl DomTokens for () {
        type UpdateState = ();

        fn update_dom_token_list_and_initialize_state(
            _: Self,
            _: &mut impl super::DomTokenList,
        ) -> Self::UpdateState {
        }

        fn update_dom_token_list(
            _: Self,
            _: &mut impl super::DomTokenList,
            _: &mut Self::UpdateState,
        ) {
        }

        type UpdateWithState = ();

        fn update_with_state(
            (): Self,
            _: &mut impl super::DomTokenList,
            (): &mut Self::UpdateWithState,
        ) {
        }

        type AsyncWritableDomTokens = NeverWritable;

        fn maybe_into_async_writable_dom_tokens((): Self) -> Option<Self::AsyncWritableDomTokens> {
            None
        }

        type DomTokensIntoAsyncStrIter = async_str_iter::never::Never;

        fn dom_tokens_maybe_into_async_str_iter(
            (): Self,
        ) -> Option<Self::DomTokensIntoAsyncStrIter> {
            None
        }
    }
}
