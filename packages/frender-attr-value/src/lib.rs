pub use self::kinds::str::AttrKindOfStr;

#[cfg(feature = "csr")]
use self::csr::macros::*;

#[cfg(feature = "csr")]
pub mod csr;
#[cfg(feature = "ssr")]
pub mod ssr;

mod kinds;

pub mod values;

#[cfg(feature = "html")]
pub mod html;

pub trait AttrValueKind: 'static + Sized {
    type AttrValue<'a>;
}

/// A marker trait.
pub trait AttrValue<VK: AttrValueKind> {}

mod known;
