use frender_common::Empty;

use frender_const::ConstUsize;

use crate::{
    constness::HasConstKnownPossibleDomTokens, dom_token::UniqueDomTokenArrayVec,
    ChainableDomTokens, DomTokens,
};

#[cfg(feature = "csr")]
mod csr {
    use frender_common::Empty;

    use crate::csr::{CsrDomTokens, DomTokenList, DomTokensStateUnmount};

    impl DomTokensStateUnmount for () {
        fn dom_tokens_state_unmount((): &mut Self, _: &mut impl DomTokenList) {}
    }

    impl CsrDomTokens for Empty {
        type State = ();

        fn dom_tokens_render_init(Self: Self, _: &mut impl DomTokenList) -> Self::State {}

        fn dom_tokens_render_init_with_old_state(
            Self: Self,
            _: &mut impl DomTokenList,
            (): &mut Self::State,
        ) {
        }

        fn dom_tokens_render_update(Self: Self, _: &mut impl DomTokenList, (): &mut Self::State) {}
    }
}

#[cfg(feature = "ssr")]
mod ssr {
    use frender_common::Empty;

    use crate::ssr::{SsrChainableDomTokens, SsrDomTokens};

    impl SsrDomTokens for Empty {
        type DomTokensIntoAsyncStrIter = async_str_iter::empty::Empty;

        fn dom_tokens_into_async_str_iter(Self: Self) -> Self::DomTokensIntoAsyncStrIter {
            async_str_iter::empty::Empty
        }
    }

    impl SsrChainableDomTokens for Empty {
        type DomTokensPrefixSpaceIntoAsyncStrIter = async_str_iter::empty::Empty;

        fn dom_tokens_prefix_space_into_async_str_iter(
            Self: Self,
        ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
            async_str_iter::empty::Empty
        }
    }
}

impl crate::sealed::DomTokens for Empty {}
impl DomTokens for Empty {}

impl crate::sealed::ChainableDomTokens for Empty {}
impl ChainableDomTokens for Empty {}

impl HasConstKnownPossibleDomTokens for Empty {
    type KnownPossibleDomTokensArrayVecCap = ConstUsize<0>;

    const KNOWN_POSSIBLE_DOM_TOKENS_ARRAY_VEC: UniqueDomTokenArrayVec<'static, 0> =
        UniqueDomTokenArrayVec::EMPTY;
}
