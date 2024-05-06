use crate::{ChainableDomTokens, DomTokens};

#[derive(Debug, Clone, Copy)]
pub struct Empty;

impl DomTokens for Empty {
    type UpdateWithState = ();

    fn update_with_state(
        Self: Self,
        _: &mut impl crate::DomTokenList,
        (): &mut Self::UpdateWithState,
    ) {
    }

    fn remove_with_state(_: &mut impl crate::DomTokenList, (): &mut Self::UpdateWithState) {}

    type DomTokensIntoAsyncStrIter = async_str_iter::empty::Empty;

    fn dom_tokens_into_async_str_iter(Self: Self) -> Self::DomTokensIntoAsyncStrIter {
        async_str_iter::empty::Empty
    }
}

impl ChainableDomTokens for Empty {
    type DomTokensPrefixSpaceIntoAsyncStrIter = async_str_iter::empty::Empty;

    fn dom_tokens_prefix_space_into_async_str_iter(
        Self: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        async_str_iter::empty::Empty
    }
}
