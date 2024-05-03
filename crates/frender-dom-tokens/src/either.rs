use async_str_iter::either::IterEither;
use either::Either;
use frender_common::either::EitherState;

use crate::DomTokens;

impl<L: DomTokens, R: DomTokens> DomTokens for Either<L, R> {
    type UpdateWithState = EitherState<L::UpdateWithState, R::UpdateWithState>;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        match this {
            Either::Left(this) => {
                let state = match state {
                    EitherState::Left { inner: state } => state,
                    EitherState::Right { inner: old_state } => {
                        R::remove_with_state(dom_token_list, old_state);
                        state.get_left_or_insert_default()
                    }
                };

                L::update_with_state(this, dom_token_list, state)
            }
            Either::Right(this) => {
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
            Either::Left(this) => IterEither::Left(L::dom_tokens_into_async_str_iter(this)),
            Either::Right(this) => IterEither::Right(R::dom_tokens_into_async_str_iter(this)),
        }
    }

    type DomTokensPrefixSpaceIntoAsyncStrIter = IterEither<
        L::DomTokensPrefixSpaceIntoAsyncStrIter,
        R::DomTokensPrefixSpaceIntoAsyncStrIter,
    >;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        match this {
            Either::Left(this) => {
                IterEither::Left(L::dom_tokens_prefix_space_into_async_str_iter(this))
            }
            Either::Right(this) => {
                IterEither::Right(R::dom_tokens_prefix_space_into_async_str_iter(this))
            }
        }
    }
}
