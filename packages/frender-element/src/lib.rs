#![cfg(all(feature = "csr", feature = "ssr"))]

use frender_csr::CsrElement;
use frender_ssr::SsrElement;

pub trait Element: SsrElement + CsrElement {}

impl<E: ?Sized + SsrElement + CsrElement> Element for E {}
