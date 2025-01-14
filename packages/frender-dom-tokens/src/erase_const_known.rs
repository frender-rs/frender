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
    type State = T::State;

    fn dom_tokens_render_init(
        Self(this): Self,
        dom_token_list: &mut impl crate::DomTokenList,
    ) -> Self::State {
        T::dom_tokens_render_init(this, dom_token_list)
    }

    fn dom_tokens_render_init_with_old_state(
        Self(this): Self,
        dom_token_list: &mut impl crate::DomTokenList,
        old_state: &mut Self::State,
    ) {
        T::dom_tokens_render_init_with_old_state(this, dom_token_list, old_state)
    }

    fn dom_tokens_render_update(
        Self(this): Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::State,
    ) {
        T::dom_tokens_render_update(this, dom_token_list, state)
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
