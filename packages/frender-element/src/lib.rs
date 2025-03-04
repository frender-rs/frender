#![cfg(all(feature = "csr", feature = "ssr"))]

pub use frender_csr::CsrElement;
pub use frender_ssr::SsrElement;

pub trait Element: SsrElement + CsrElement {}

impl<E: ?Sized + SsrElement + CsrElement> Element for E {}
