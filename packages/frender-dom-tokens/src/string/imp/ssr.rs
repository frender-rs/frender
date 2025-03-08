use crate::ssr::SsrDomTokens;

use super::super::KnownNonReactiveStr;
use super::NonReactiveStrIntoDomTokens;

impl<S: KnownNonReactiveStr> SsrDomTokens for NonReactiveStrIntoDomTokens<S> {
    type DomTokensIntoAsyncStrIter = S::SsrStrIntoAsyncStrIterator;

    fn dom_tokens_into_async_str_iter(this: Self) -> Self::DomTokensIntoAsyncStrIter {
        S::ssr_str_into_async_str_iterator(this.0)
    }
}
