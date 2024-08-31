use async_str_iter::either::IterEither;
use frender_common::either::EitherState;
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

impl<L: DomTokens, R: DomTokens> DomTokens for EitherDomTokens<L, R> {
    type UpdateWithState = EitherState<L::UpdateWithState, R::UpdateWithState>;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        match this {
            EitherDomTokens::A(this) => {
                let state = match state {
                    EitherState::Left { inner: state } => state,
                    EitherState::Right { inner: old_state } => {
                        R::remove_with_state(dom_token_list, old_state);
                        state.get_left_or_insert_default()
                    }
                };

                L::update_with_state(this, dom_token_list, state)
            }
            EitherDomTokens::B(this) => {
                let state = match state {
                    EitherState::Right { inner: state } => state,
                    EitherState::Left { inner: old_state } => {
                        L::remove_with_state(dom_token_list, old_state);
                        state.get_right_or_insert_default()
                    }
                };

                R::update_with_state(this, dom_token_list, state)
            }
        }
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        match state {
            EitherState::Left { inner: state } => L::remove_with_state(dom_token_list, state),
            EitherState::Right { inner: state } => R::remove_with_state(dom_token_list, state),
        }
    }

    type DomTokensIntoAsyncStrIter =
        IterEither<L::DomTokensIntoAsyncStrIter, R::DomTokensIntoAsyncStrIter>;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        match this {
            EitherDomTokens::A(this) => IterEither::Left(L::dom_tokens_into_async_str_iter(this)),
            EitherDomTokens::B(this) => IterEither::Right(R::dom_tokens_into_async_str_iter(this)),
        }
    }
}

impl<L: ChainableDomTokens, R: ChainableDomTokens> ChainableDomTokens for EitherDomTokens<L, R>
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
