use async_str_iter::AsyncStrIterator;

use crate::{ChainableDomTokens, IntoDomTokens};

pub trait SsrDomTokens {
    // TODO: typed templating
    type DomTokensIntoAsyncStrIter: AsyncStrIterator;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter;
}

pub trait SsrChainableDomTokens {
    type DomTokensPrefixSpaceIntoAsyncStrIter: asserts::DomTokensPrefixSpace;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter;
}

impl<T: IntoDomTokens> SsrDomTokens for T {
    type DomTokensIntoAsyncStrIter = <T::IntoDomTokens as SsrDomTokens>::DomTokensIntoAsyncStrIter;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        <T::IntoDomTokens>::dom_tokens_into_async_str_iter(this.into_dom_tokens())
    }
}

impl<T: IntoDomTokens> SsrChainableDomTokens for T
where
    T::IntoDomTokens: ChainableDomTokens,
{
    type DomTokensPrefixSpaceIntoAsyncStrIter =
        <T::IntoDomTokens as SsrChainableDomTokens>::DomTokensPrefixSpaceIntoAsyncStrIter;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        <T::IntoDomTokens as SsrChainableDomTokens>::dom_tokens_prefix_space_into_async_str_iter(
            this.into_dom_tokens(),
        )
    }
}

pub mod asserts {
    use async_str_iter::AsyncStrIterator;

    mod sealed {
        pub trait DomTokensPrefixSpace {}
    }

    /// [Empty](async_str_iter::empty::Empty) or multiple space separated dom tokens prefixed with a space.
    pub trait DomTokensPrefixSpace: AsyncStrIterator + sealed::DomTokensPrefixSpace {}

    // empty
    impl sealed::DomTokensPrefixSpace for async_str_iter::empty::Empty {}
    impl DomTokensPrefixSpace for async_str_iter::empty::Empty {}

    // never
    impl sealed::DomTokensPrefixSpace for async_str_iter::never::Never {}
    impl DomTokensPrefixSpace for async_str_iter::never::Never {}

    // option
    impl<T: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
        for async_str_iter::option::IterOption<T>
    {
    }
    impl<T: DomTokensPrefixSpace> DomTokensPrefixSpace for async_str_iter::option::IterOption<T> {}

    // either
    impl<L: DomTokensPrefixSpace, R: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
        for async_str_iter::either::IterEither<L, R>
    {
    }
    impl<L: DomTokensPrefixSpace, R: DomTokensPrefixSpace> DomTokensPrefixSpace
        for async_str_iter::either::IterEither<L, R>
    {
    }

    // chain
    impl<A: DomTokensPrefixSpace, B: DomTokensPrefixSpace> sealed::DomTokensPrefixSpace
        for async_str_iter::chain::Chain<A, B>
    {
    }
    impl<A: DomTokensPrefixSpace, B: DomTokensPrefixSpace> DomTokensPrefixSpace
        for async_str_iter::chain::Chain<A, B>
    {
    }

    // const
    impl<T: ?Sized + crate::constness::HasConstDomTokens> sealed::DomTokensPrefixSpace
        for crate::constness::ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
    {
    }
    impl<T: ?Sized + crate::constness::HasConstDomTokens> DomTokensPrefixSpace
        for crate::constness::ssr::ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
    {
    }
}
