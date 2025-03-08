#[cfg(feature = "csr")]
use crate::csr::CsrDomTokens;
#[cfg(feature = "ssr")]
use crate::ssr::{SsrChainableDomTokens, SsrDomTokens};

#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
pub trait DomTokens: CsrDomTokens + SsrDomTokens {}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
pub trait DomTokens: CsrDomTokens {}

#[cfg(not(feature = "csr"))]
#[cfg(feature = "ssr")]
pub trait DomTokens: SsrDomTokens {}

#[cfg(not(feature = "csr"))]
#[cfg(not(feature = "ssr"))]
pub trait DomTokens {}

#[cfg(feature = "ssr")]
pub trait ChainableDomTokens: SsrChainableDomTokens {}

#[cfg(not(feature = "ssr"))]
pub trait ChainableDomTokens {}
