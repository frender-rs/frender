use frender_common::Empty;

use frender_const::ConstUsize;

use crate::{
    constness::HasConstKnownPossibleDomTokens, dom_token::UniqueDomTokenArrayVec,
    ChainableDomTokens, DomTokens, DomTokensStateUnmount,
};

impl DomTokensStateUnmount for () {
    fn dom_tokens_state_unmount((): &mut Self, _: &mut impl crate::DomTokenList) {}
}

impl DomTokens for Empty {
    type State = ();

    fn dom_tokens_render_init(Self: Self, _: &mut impl crate::DomTokenList) -> Self::State {}

    fn dom_tokens_render_init_with_old_state(
        Self: Self,
        _: &mut impl crate::DomTokenList,
        (): &mut Self::State,
    ) {
    }

    fn dom_tokens_render_update(
        Self: Self,
        _: &mut impl crate::DomTokenList,
        (): &mut Self::State,
    ) {
    }

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

impl HasConstKnownPossibleDomTokens for Empty {
    type KnownPossibleDomTokensArrayVecCap = ConstUsize<0>;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, 0> =
        UniqueDomTokenArrayVec::EMPTY;
}
