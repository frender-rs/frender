use async_str_iter::{option::IterOption, IntoAsyncStrIterator};

use crate::{ChainableDomTokens, DomTokens};

impl<T: DomTokens> DomTokens for Option<T> {
    type UpdateWithState = T::UpdateWithState;

    fn update_with_state(
        this: Self,
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        if let Some(this) = this {
            T::update_with_state(this, dom_token_list, state)
        } else {
            T::remove_with_state(dom_token_list, state)
        }
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        state: &mut Self::UpdateWithState,
    ) {
        T::remove_with_state(dom_token_list, state)
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
