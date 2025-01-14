use frender_const::{ConstUsize, KnownConstUsizeAdd};

use crate::{
    constness::{HasConstKnownPossibleDomTokens, IsConstUsize},
    dom_token::UniqueDomTokenArrayVec,
    ChainableDomTokens, DomTokens, DomTokensStateUnmount,
};

#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);

impl<A, B> Chain<A, B> {
    pub const fn new(a: A, b: B) -> Self {
        Self(a, b)
    }
}

impl<A, B> Chain<A, B>
where
    Self: HasConstKnownPossibleDomTokens,
{
    pub const ASSERT_KNOWN_NOT_DUP: () = {
        _ = Self::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC;
    };
}

impl<A: DomTokensStateUnmount, B: DomTokensStateUnmount> DomTokensStateUnmount for (A, B) {
    fn dom_tokens_state_unmount(
        (state_a, state_b): &mut Self,
        dom_token_list: &mut impl crate::DomTokenList,
    ) {
        A::dom_tokens_state_unmount(state_a, dom_token_list);
        B::dom_tokens_state_unmount(state_b, dom_token_list);
    }
}

impl<A: ChainableDomTokens, B: ChainableDomTokens> DomTokens for Chain<A, B>
where
    Self: HasConstKnownPossibleDomTokens,
{
    type State = (A::State, B::State);

    fn dom_tokens_render_init(
        Self(a, b): Self,
        dom_token_list: &mut impl crate::DomTokenList,
    ) -> Self::State {
        _ = Self::ASSERT_KNOWN_NOT_DUP;
        (
            A::dom_tokens_render_init(a, dom_token_list),
            B::dom_tokens_render_init(b, dom_token_list),
        )
    }

    fn dom_tokens_render_init_with_old_state(
        Self(a, b): Self,
        dom_token_list: &mut impl crate::DomTokenList,
        (old_state_a, old_state_b): &mut Self::State,
    ) {
        _ = Self::ASSERT_KNOWN_NOT_DUP;
        A::dom_tokens_render_init_with_old_state(a, dom_token_list, old_state_a);
        B::dom_tokens_render_init_with_old_state(b, dom_token_list, old_state_b);
    }

    fn dom_tokens_render_update(
        Self(a, b): Self,
        dom_token_list: &mut impl crate::DomTokenList,
        (state_a, state_b): &mut Self::State,
    ) {
        _ = Self::ASSERT_KNOWN_NOT_DUP;
        A::dom_tokens_render_update(a, dom_token_list, state_a);
        B::dom_tokens_render_update(b, dom_token_list, state_b);
    }

    type DomTokensIntoAsyncStrIter = async_str_iter::chain::Chain<
        A::DomTokensIntoAsyncStrIter,
        B::DomTokensPrefixSpaceIntoAsyncStrIter,
    >;

    fn dom_tokens_into_async_str_iter(Self(a, b): Self) -> Self::DomTokensIntoAsyncStrIter {
        _ = Self::ASSERT_KNOWN_NOT_DUP;
        async_str_iter::chain::Chain::new(
            A::dom_tokens_into_async_str_iter(a),
            B::dom_tokens_prefix_space_into_async_str_iter(b),
        )
    }
}

impl<A: ChainableDomTokens, B: ChainableDomTokens> ChainableDomTokens for Chain<A, B>
where
    Self: HasConstKnownPossibleDomTokens,
{
    type DomTokensPrefixSpaceIntoAsyncStrIter = async_str_iter::chain::Chain<
        A::DomTokensPrefixSpaceIntoAsyncStrIter,
        B::DomTokensPrefixSpaceIntoAsyncStrIter,
    >;

    fn dom_tokens_prefix_space_into_async_str_iter(
        Self(a, b): Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        _ = Self::ASSERT_KNOWN_NOT_DUP;
        async_str_iter::chain::Chain::new(
            A::dom_tokens_prefix_space_into_async_str_iter(a),
            B::dom_tokens_prefix_space_into_async_str_iter(b),
        )
    }
}

impl<
        const M: usize,
        const N: usize,
        const SUM: usize,
        A: HasConstKnownPossibleDomTokens,
        B: HasConstKnownPossibleDomTokens<KnownPossibleDomTokensArrayVecCap = ConstUsize<N>>,
    > HasConstKnownPossibleDomTokens for Chain<A, B>
where
    A::KnownPossibleDomTokensArrayVecCap: IsConstUsize<UniqueDomTokenArrayVec<'static> = UniqueDomTokenArrayVec<'static, M>>
        + KnownConstUsizeAdd<ConstUsize<N>, Sum = ConstUsize<SUM>>,
{
    type KnownPossibleDomTokensArrayVecCap = ConstUsize<SUM>;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, SUM> = {
        A::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC
            .with_capacity::<SUM>()
            .with_extend_unique_dom_tokens(
                B::KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC.as_unique_dom_tokens(),
            )
    };
}
