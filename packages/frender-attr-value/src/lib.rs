pub use self::kinds::str::AttrKindOfStr;

#[cfg(feature = "csr")]
use self::csr::macros::*;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

mod kinds;

pub mod values;

#[cfg(feature = "html")]
pub mod html;

pub trait AttrValueKind: 'static + Sized {
    type AttrValue<'a>;
}

mod sealed {
    use crate::AttrValueKind;

    #[cfg(feature = "csr")]
    use super::csr::CsrAttrValue;
    #[cfg(feature = "ssr")]
    use super::ssr::SsrAttrValue;

    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub trait AttrValue<VK: AttrValueKind>: CsrAttrValue<VK> + SsrAttrValue<VK> {}

    #[cfg(feature = "csr")]
    #[cfg(not(feature = "ssr"))]
    pub trait AttrValue<VK: AttrValueKind>: CsrAttrValue<VK> {}

    #[cfg(not(feature = "csr"))]
    #[cfg(feature = "ssr")]
    pub trait AttrValue<VK: AttrValueKind>: SsrAttrValue<VK> {}

    #[cfg(not(feature = "csr"))]
    #[cfg(not(feature = "ssr"))]
    pub trait AttrValue<VK: AttrValueKind> {}
}

pub trait AttrValue<VK: AttrValueKind>: sealed::AttrValue<VK> {}

pub trait ImplAttrValueWithIntoAttrValue {}

pub trait IntoAttrValue<VK: AttrValueKind>: ImplAttrValueWithIntoAttrValue {
    type IntoAttrValue: AttrValue<VK>;

    fn into_attr_value(self) -> Self::IntoAttrValue;
}

impl<T: IntoAttrValue<VK>, VK: AttrValueKind> sealed::AttrValue<VK> for T {}
impl<T: IntoAttrValue<VK>, VK: AttrValueKind> AttrValue<VK> for T {}

mod known;

pub mod attr_value;
