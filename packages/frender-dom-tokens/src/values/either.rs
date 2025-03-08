use frender_const::{ConstUsize, KnownConstUsizeAdd};

use crate::{
    constness::{HasConstKnownPossibleDomTokens, IsConstUsize},
    dom_token::UniqueDomTokenArrayVec,
    ChainableDomTokens, DomTokens,
};

#[derive(Debug, Clone, Copy)]
pub enum EitherDomTokens<A, B> {
    A(A),
    B(B),
}

#[cfg(feature = "csr")]
mod csr {
    use crate::{
        csr::{CsrDomTokens, DomTokenList, DomTokensStateUnmount},
        DomTokens,
    };

    use super::EitherDomTokens;

    pub enum State<A, B> {
        A(A),
        B(B),
    }

    impl<A: DomTokensStateUnmount, B: DomTokensStateUnmount> DomTokensStateUnmount for State<A, B> {
        fn dom_tokens_state_unmount(state: &mut Self, dom_token_list: &mut impl DomTokenList) {
            match state {
                State::A(state) => A::dom_tokens_state_unmount(state, dom_token_list),
                State::B(state) => B::dom_tokens_state_unmount(state, dom_token_list),
            }
        }
    }

    impl<L: DomTokens, R: DomTokens> CsrDomTokens for EitherDomTokens<L, R> {
        type State = State<L::State, R::State>;

        fn dom_tokens_render_init(
            this: Self,
            dom_token_list: &mut impl DomTokenList,
        ) -> Self::State {
            match this {
                EitherDomTokens::A(this) => {
                    State::A(L::dom_tokens_render_init(this, dom_token_list))
                }
                EitherDomTokens::B(this) => {
                    State::B(R::dom_tokens_render_init(this, dom_token_list))
                }
            }
        }

        fn dom_tokens_render_init_with_old_state(
            this: Self,
            dom_token_list: &mut impl DomTokenList,
            old_state: &mut Self::State,
        ) {
            match this {
                EitherDomTokens::A(this) => match old_state {
                    State::A(old_state) => {
                        L::dom_tokens_render_init_with_old_state(this, dom_token_list, old_state)
                    }
                    State::B(_) => {
                        // Already unmounted
                        *old_state = State::A(L::dom_tokens_render_init(this, dom_token_list))
                    }
                },
                EitherDomTokens::B(this) => match old_state {
                    State::B(old_state) => {
                        R::dom_tokens_render_init_with_old_state(this, dom_token_list, old_state)
                    }
                    State::A(_) => {
                        // Already unmounted
                        *old_state = State::B(R::dom_tokens_render_init(this, dom_token_list))
                    }
                },
            }
        }

        fn dom_tokens_render_update(
            this: Self,
            dom_token_list: &mut impl DomTokenList,
            state: &mut Self::State,
        ) {
            match this {
                EitherDomTokens::A(this) => match state {
                    State::A(state) => L::dom_tokens_render_update(this, dom_token_list, state),
                    State::B(old_state) => {
                        <R::State>::dom_tokens_state_unmount(old_state, dom_token_list);
                        *state = State::A(L::dom_tokens_render_init(this, dom_token_list))
                    }
                },
                EitherDomTokens::B(this) => match state {
                    State::B(state) => R::dom_tokens_render_update(this, dom_token_list, state),
                    State::A(old_state) => {
                        <L::State>::dom_tokens_state_unmount(old_state, dom_token_list);
                        *state = State::B(R::dom_tokens_render_init(this, dom_token_list))
                    }
                },
            }
        }
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use async_str_iter::either::IterEither;

    use crate::{
        constness::HasConstKnownPossibleDomTokens,
        ssr::{SsrChainableDomTokens, SsrDomTokens},
        ChainableDomTokens, DomTokens,
    };

    use super::EitherDomTokens;

    impl<L: DomTokens, R: DomTokens> SsrDomTokens for EitherDomTokens<L, R> {
        type DomTokensIntoAsyncStrIter =
            IterEither<L::DomTokensIntoAsyncStrIter, R::DomTokensIntoAsyncStrIter>;

        fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
            match this {
                EitherDomTokens::A(this) => {
                    IterEither::Left(L::dom_tokens_into_async_str_iter(this))
                }
                EitherDomTokens::B(this) => {
                    IterEither::Right(R::dom_tokens_into_async_str_iter(this))
                }
            }
        }
    }

    impl<L: ChainableDomTokens, R: ChainableDomTokens> SsrChainableDomTokens for EitherDomTokens<L, R>
    where
        Self: HasConstKnownPossibleDomTokens,
    {
        type DomTokensPrefixSpaceIntoAsyncStrIter = IterEither<
            L::DomTokensPrefixSpaceIntoAsyncStrIter,
            R::DomTokensPrefixSpaceIntoAsyncStrIter,
        >;

        fn dom_tokens_prefix_space_into_async_str_iter(
            this: Self,
        ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
            match this {
                EitherDomTokens::A(this) => {
                    IterEither::Left(L::dom_tokens_prefix_space_into_async_str_iter(this))
                }
                EitherDomTokens::B(this) => {
                    IterEither::Right(R::dom_tokens_prefix_space_into_async_str_iter(this))
                }
            }
        }
    }
}

impl<L: DomTokens, R: DomTokens> crate::sealed::DomTokens for EitherDomTokens<L, R> {}
impl<L: DomTokens, R: DomTokens> DomTokens for EitherDomTokens<L, R> {}

impl<A: ChainableDomTokens, B: ChainableDomTokens> crate::sealed::ChainableDomTokens
    for EitherDomTokens<A, B>
where
    Self: HasConstKnownPossibleDomTokens,
{
}
impl<L: ChainableDomTokens, R: ChainableDomTokens> ChainableDomTokens for EitherDomTokens<L, R> where
    Self: HasConstKnownPossibleDomTokens
{
}

impl<
        L: HasConstKnownPossibleDomTokens,
        R: HasConstKnownPossibleDomTokens<KnownPossibleDomTokensArrayVecCap = ConstUsize<N>>,
        const M: usize,
        const N: usize,
        const SUM: usize,
    > HasConstKnownPossibleDomTokens for EitherDomTokens<L, R>
where
    L::KnownPossibleDomTokensArrayVecCap: IsConstUsize<UniqueDomTokenArrayVec<'static> = UniqueDomTokenArrayVec<'static, M>>
        + KnownConstUsizeAdd<ConstUsize<N>, Sum = ConstUsize<SUM>>,
{
    type KnownPossibleDomTokensArrayVecCap = ConstUsize<SUM>;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, SUM> = {
        L::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC
            .with_capacity::<SUM>()
            .with_extend_unique_dom_tokens_and_remove_duplicated(
                R::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_unique_dom_tokens(),
            )
    };
}

#[cfg(feature = "either")]
mod extern_either {
    use either::Either;

    use crate::{DomTokens, IntoDomTokens};

    use super::EitherDomTokens;

    impl<L: DomTokens, R: DomTokens> IntoDomTokens for Either<L, R> {
        type IntoDomTokens = EitherDomTokens<L, R>;

        fn into_dom_tokens(self) -> Self::IntoDomTokens {
            match self {
                Either::Left(this) => EitherDomTokens::A(this),
                Either::Right(this) => EitherDomTokens::B(this),
            }
        }
    }
}
