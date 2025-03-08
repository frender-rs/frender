use crate::DomTokens;

use super::KnownNonReactiveStr;

pub struct NonReactiveStrIntoDomTokens<S>(pub(super) S);

impl<S: KnownNonReactiveStr> crate::sealed::DomTokens for NonReactiveStrIntoDomTokens<S> {}
impl<S: KnownNonReactiveStr> DomTokens for NonReactiveStrIntoDomTokens<S> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
