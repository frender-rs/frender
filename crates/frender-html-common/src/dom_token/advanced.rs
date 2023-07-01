use super::*;

/// `S` should NOT contain ascii whitespace,
/// or there would be runtime error when this token is used.
pub struct OneDomToken<S>(pub S);
pub struct DomTokensWithPredicate<T: DomTokens>(pub T, pub bool);

#[macro_export]
macro_rules! __dom_token {
    ($lit:literal) => {
        $lit
    };
    ($ident:ident) => {
        $ident
    };
    ({$expr:expr}) => {
        $expr
    };
}

#[macro_export]
macro_rules! dom_token {
    ($dom_token:tt = $predicate:expr) => {
        $crate::dom_token::DomTokenWithPredicate($crate::__dom_token!($dom_token), $predicate)
    };
    ($dom_token:tt) => {
        $crate::dom_token::DomToken($crate::__dom_token!($dom_token))
    };
}

pub mod chain {
    use super::DomTokens;

    pub struct DomTokensChain<A: DomTokens, B: DomTokens<StringChunk = A::StringChunk>>(
        pub A,
        pub B,
    );

    impl<A: DomTokens, B: DomTokens<StringChunk = A::StringChunk>> DomTokens for DomTokensChain<A, B> {
        type UpdateState = (A::UpdateState, B::UpdateState);

        fn update_dom_token_list_and_initialize_state(
            this: Self,
            dom_token_list: &impl super::DomTokenList,
        ) -> Self::UpdateState {
            (
                A::update_dom_token_list_and_initialize_state(this.0, dom_token_list),
                B::update_dom_token_list_and_initialize_state(this.1, dom_token_list),
            )
        }

        fn update_dom_token_list(
            this: Self,
            dom_token_list: &impl super::DomTokenList,
            state: &mut Self::UpdateState,
        ) {
            A::update_dom_token_list(this.0, dom_token_list, &mut state.0);
            B::update_dom_token_list(this.1, dom_token_list, &mut state.1);
        }

        type StringChunk = A::StringChunk;

        type StringChunks = core::iter::Chain<A::StringChunks, B::StringChunks>;

        fn into_string_chunks(this: Self) -> Self::StringChunks {
            A::into_string_chunks(this.0).chain(B::into_string_chunks(this.1))
        }
    }
}

pub mod chain_either {
    use either::Either;

    use super::DomTokens;

    pub struct DomTokensChainEither<A: DomTokens, B: DomTokens>(pub A, pub B);

    #[derive(Clone, Debug)]
    pub struct ChainEither<A, B> {
        a: Option<A>,
        b: B,
    }

    impl<A: Iterator, B: Iterator> Iterator for ChainEither<A, B> {
        type Item = Either<A::Item, B::Item>;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(ref mut a) = self.a {
                if let Some(a) = a.next() {
                    return Some(Either::Left(a));
                } else {
                    self.a = None;
                }
            }

            // self.b.as_mut().and_then(B::next).map(Either::Right)
            self.b.next().map(Either::Right)
        }
    }

    impl<A: DomTokens, B: DomTokens> DomTokens for DomTokensChainEither<A, B> {
        type UpdateState = (A::UpdateState, B::UpdateState);

        fn update_dom_token_list_and_initialize_state(
            this: Self,
            dom_token_list: &impl super::DomTokenList,
        ) -> Self::UpdateState {
            (
                A::update_dom_token_list_and_initialize_state(this.0, dom_token_list),
                B::update_dom_token_list_and_initialize_state(this.1, dom_token_list),
            )
        }

        fn update_dom_token_list(
            this: Self,
            dom_token_list: &impl super::DomTokenList,
            state: &mut Self::UpdateState,
        ) {
            A::update_dom_token_list(this.0, dom_token_list, &mut state.0);
            B::update_dom_token_list(this.1, dom_token_list, &mut state.1);
        }

        type StringChunk = Either<A::StringChunk, B::StringChunk>;

        type StringChunks = ChainEither<A::StringChunks, B::StringChunks>;

        fn into_string_chunks(this: Self) -> Self::StringChunks {
            ChainEither {
                a: Some(A::into_string_chunks(this.0)),
                b: B::into_string_chunks(this.1),
            }
        }
    }
}

pub mod auto_chain {
    use super::DomTokens;

    pub struct AutoChain<A: DomTokens, B: DomTokens>(pub A, pub B);

    pub struct TagChain;

    impl TagChain {
        pub fn auto_chain<A: DomTokens, B: DomTokens<StringChunk = A::StringChunk>>(
            self,
            AutoChain(a, b): AutoChain<A, B>,
        ) -> super::chain::DomTokensChain<A, B> {
            super::chain::DomTokensChain(a, b)
        }
    }

    pub struct TagChainEither;

    impl TagChainEither {
        pub fn auto_chain<A: DomTokens, B: DomTokens>(
            self,
            AutoChain(a, b): AutoChain<A, B>,
        ) -> super::chain_either::DomTokensChainEither<A, B> {
            super::chain_either::DomTokensChainEither(a, b)
        }
    }

    pub trait AutoChainTag {
        type Tag;
        fn dom_tokens_auto_chain_tag(&self) -> Self::Tag;
    }

    /// Specialize when `A::StringChunk == B::StringChunk`
    impl<A: DomTokens, B: DomTokens<StringChunk = A::StringChunk>> AutoChainTag for AutoChain<A, B> {
        type Tag = TagChain;

        fn dom_tokens_auto_chain_tag(&self) -> Self::Tag {
            TagChain
        }
    }

    impl<A: DomTokens, B: DomTokens> AutoChainTag for &AutoChain<A, B> {
        type Tag = TagChainEither;

        fn dom_tokens_auto_chain_tag(&self) -> Self::Tag {
            TagChainEither
        }
    }
}

#[macro_export]
macro_rules! auto_chain {
    ($a:expr, $b:expr) => {
        match $crate::dom_token::auto_chain::AutoChain($a, $b) {
            c => {
                use $crate::dom_token::auto_chain::AutoChainTag as _;
                c.dom_tokens_auto_chain_tag().auto_chain(c)
            }
        }
    };
}
