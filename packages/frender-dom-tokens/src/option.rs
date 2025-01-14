use async_str_iter::{option::IterOption, IntoAsyncStrIterator};

use crate::{
    constness::{HasConstKnownPossibleDomTokens, IsConstUsize},
    ChainableDomTokens, DomTokens, DomTokensStateUnmount,
};

impl<T: DomTokensStateUnmount> DomTokensStateUnmount for Option<T> {
    fn dom_tokens_state_unmount(state: &mut Self, dom_token_list: &mut impl crate::DomTokenList) {
        if let Some(state) = state {
            T::dom_tokens_state_unmount(state, dom_token_list);
        }
        *state = None; // drop the old state
    }
}

impl<T: DomTokens> DomTokens for Option<T> {
    type State = Option<T::State>;

    fn dom_tokens_render_init(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
    ) -> Self::State {
        if let Some(this) = this {
            Some(T::dom_tokens_render_init(this, dom_token_list))
        } else {
            None
        }
    }

    // old_state must have been set to None in its dom_tokens_state_unmount.
    // So we can just use the default implementation for
    // fn dom_tokens_render_init_with_old_state

    fn dom_tokens_render_update(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::State,
    ) {
        if let Some(this) = this {
            if let Some(state) = state {
                T::dom_tokens_render_update(this, dom_token_list, state)
            } else {
                *state = Some(T::dom_tokens_render_init(this, dom_token_list))
            }
        } else {
            Self::State::dom_tokens_state_unmount(state, dom_token_list)
        }
    }

    type DomTokensIntoAsyncStrIter = IterOption<T::DomTokensIntoAsyncStrIter>;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        this.map(T::dom_tokens_into_async_str_iter)
            .into_async_str_iterator()
    }
}

impl<T: ChainableDomTokens> ChainableDomTokens for Option<T> {
    type DomTokensPrefixSpaceIntoAsyncStrIter = IterOption<T::DomTokensPrefixSpaceIntoAsyncStrIter>;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        this.map(T::dom_tokens_prefix_space_into_async_str_iter)
            .into_async_str_iterator()
    }
}

impl<T: HasConstKnownPossibleDomTokens> HasConstKnownPossibleDomTokens for Option<T> {
    type KnownPossibleDomTokensArrayVecCap = T::KnownPossibleDomTokensArrayVecCap;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC:
        <T::KnownPossibleDomTokensArrayVecCap as IsConstUsize>::UniqueDomTokenArrayVec<'static> =
        T::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC;
}
