#[cfg(feature = "csr")]
pub use frender_csr::CsrElement;
#[cfg(feature = "ssr")]
pub use frender_ssr::SsrElement;

#[cfg(all(feature = "csr", feature = "ssr"))]
/// A trait alias for [`SsrElement`] + [`CsrElement`]
pub trait Element: SsrElement + CsrElement {}

#[cfg(all(feature = "csr", feature = "ssr"))]
impl<E: ?Sized + SsrElement + CsrElement> Element for E {}
