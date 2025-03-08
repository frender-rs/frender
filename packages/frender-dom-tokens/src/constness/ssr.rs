use std::task::Poll;

use async_str_iter::AsyncStrIterator;
use frender_const::ConstUsize;

use crate::ssr::{SsrChainableDomTokens, SsrDomTokens};

use super::{ConstDomTokens, HasConstDomTokens};

pub struct ConstDomTokensIntoAsyncStrIter<T: ?Sized + HasConstDomTokens> {
    pub(crate) _const: ConstDomTokens<T>,
    pub(crate) yielded: bool,
}

impl<T: ?Sized + HasConstDomTokens> Unpin for ConstDomTokensIntoAsyncStrIter<T> {}

impl<T: ?Sized + HasConstDomTokens> AsyncStrIterator for ConstDomTokensIntoAsyncStrIter<T> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<&str>> {
        Poll::Ready({
            let this = self.get_mut();
            if this.yielded {
                None
            } else {
                this.yielded = true;
                Some(T::DOM_TOKENS_PREFIX_SPACE.to_str_without_prefix_space())
            }
        })
    }
}

pub struct ConstDomTokensPrefixSpaceIntoAsyncStrIter<T: ?Sized + HasConstDomTokens> {
    pub(crate) _const: ConstDomTokens<T>,
    pub(crate) yielded: bool,
}

impl<T: ?Sized + HasConstDomTokens> Unpin for ConstDomTokensPrefixSpaceIntoAsyncStrIter<T> {}

impl<T: ?Sized + HasConstDomTokens> AsyncStrIterator
    for ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>
{
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> Poll<Option<&str>> {
        Poll::Ready({
            let this = self.get_mut();
            if this.yielded {
                None
            } else {
                this.yielded = true;
                Some(T::DOM_TOKENS_PREFIX_SPACE.to_str())
            }
        })
    }
}

impl<T: ?Sized + HasConstDomTokens> SsrDomTokens for ConstDomTokens<T> {
    type DomTokensIntoAsyncStrIter = ConstDomTokensIntoAsyncStrIter<T>;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        ConstDomTokensIntoAsyncStrIter {
            _const: this,
            yielded: false,
        }
    }
}

impl<T: ?Sized + HasConstDomTokens<DomTokensLen = ConstUsize<N>>, const N: usize>
    SsrChainableDomTokens for ConstDomTokens<T>
{
    type DomTokensPrefixSpaceIntoAsyncStrIter = ConstDomTokensPrefixSpaceIntoAsyncStrIter<T>;

    fn dom_tokens_prefix_space_into_async_str_iter(
        this: Self,
    ) -> Self::DomTokensPrefixSpaceIntoAsyncStrIter {
        ConstDomTokensPrefixSpaceIntoAsyncStrIter {
            _const: this,
            yielded: false,
        }
    }
}

impl<'a> super::DomTokensPrefixSpaceStr<'a> {
    const fn to_str(self) -> &'a str {
        self.inner
    }

    fn to_str_without_prefix_space(self) -> &'a str {
        &self.to_str()[1..]
    }
}
