use frender_const::ConstUsize;

use crate::{
    constness::HasConstKnownPossibleDomTokens, ChainableDomTokens, DomTokens,
    UniqueDomTokenArrayVec,
};

/// Always impl [`HasConstKnownPossibleDomTokens`] with no dom tokens,
/// indicating no dom tokens is known at compile time.
/// This is useful when chaining generic [`DomTokens`].
#[derive(Debug, Clone, Copy)]
pub struct EraseConstKnownPossibleDomTokens<T>(pub T);

impl<T: DomTokens> DomTokens for EraseConstKnownPossibleDomTokens<T> {
    type UpdateWithState = T::UpdateWithState;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        T::update_with_state(this.0, dom_token_list, state);
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        T::remove_with_state(dom_token_list, state);
    }

    type DomTokensIntoAsyncStrIter = T::DomTokensIntoAsyncStrIter;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        T::dom_tokens_into_async_str_iter(this.0)
    }
}

impl<T> HasConstKnownPossibleDomTokens for EraseConstKnownPossibleDomTokens<T> {
    type KnownPossibleDomTokensArrayVecCap = ConstUsize<0>;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, 0> =
        UniqueDomTokenArrayVec::EMPTY;
}

impl<T: ChainableDomTokens> ChainableDomTokens for EraseConstKnownPossibleDomTokens<T> {
    type DomTokensPrefixSpaceIntoAsyncStrIter = T::DomTokensPrefixSpaceIntoAsyncStrIter;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        T::dom_tokens_prefix_space_into_async_str_iter(this.0)
    }
}
