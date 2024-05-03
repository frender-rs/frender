use crate::DomTokens;

#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);

impl<A, B> Chain<A, B> {
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A: DomTokens, B: DomTokens> DomTokens for Chain<A, B> {
    type UpdateWithState = (A::UpdateWithState, B::UpdateWithState);

    fn update_with_state(
        Self(a, b): Self,
        dom_token_list: &mut impl crate::DomTokenList,
        (state_a, state_b): &mut Self::UpdateWithState,
    ) {
        A::update_with_state(a, dom_token_list, state_a);
        B::update_with_state(b, dom_token_list, state_b);
    }

    fn remove_with_state(
        dom_token_list: &mut impl crate::DomTokenList,
        (state_a, state_b): &mut Self::UpdateWithState,
    ) {
        A::remove_with_state(dom_token_list, state_a);
        B::remove_with_state(dom_token_list, state_b);
    }

    type DomTokensIntoAsyncStrIter = async_str_iter::chain::Chain<
        A::DomTokensIntoAsyncStrIter,
        B::DomTokensPrefixSpaceIntoAsyncStrIter,
    >;

    fn dom_tokens_into_async_str_iter(Self(a, b): Self) -> Self::DomTokensIntoAsyncStrIter {
        async_str_iter::chain::Chain::new(
            A::dom_tokens_into_async_str_iter(a),
            B::dom_tokens_prefix_space_into_async_str_iter(b),
        )
    }

    type DomTokensPrefixSpaceIntoAsyncStrIter = async_str_iter::chain::Chain<
        A::DomTokensPrefixSpaceIntoAsyncStrIter,
        B::DomTokensPrefixSpaceIntoAsyncStrIter,
    >;

    fn dom_tokens_prefix_space_into_async_str_iter(
        Self(a, b): Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        async_str_iter::chain::Chain::new(
            A::dom_tokens_prefix_space_into_async_str_iter(a),
            B::dom_tokens_prefix_space_into_async_str_iter(b),
        )
    }
}
