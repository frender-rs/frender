pub use self::kinds::str::AttrKindOfStr;

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

pub trait IntoAttrValue<VK: AttrValueKind> {
    type IntoAttrValue: AttrValue<VK>;

    fn into_attr_value(self) -> Self::IntoAttrValue;
}

mod known;

pub mod attr_value;

#[cfg(feature = "experimental")]
pub mod experimental;
