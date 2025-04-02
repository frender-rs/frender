pub mod values;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

pub mod attrs;

/// Non reactive attributes with unpinned state.
pub trait IntoAttributes {
    type IntoAttributes: Attributes;
    fn into_attributes(self) -> Self::IntoAttributes;
}

pub trait Attributes: sealed::Attributes {}

mod sealed {
    #[cfg(feature = "csr")]
    use super::csr::CsrAttributes;

    #[cfg(feature = "ssr")]
    use super::ssr::SsrAttributes;

    #[cfg(feature = "csr")]
    #[cfg(feature = "ssr")]
    pub trait Attributes: CsrAttributes + SsrAttributes {}

    #[cfg(feature = "csr")]
    #[cfg(not(feature = "ssr"))]
    pub trait Attributes: CsrAttributes {}

    #[cfg(not(feature = "csr"))]
    #[cfg(feature = "ssr")]
    pub trait Attributes: SsrAttributes {}

    #[cfg(not(feature = "csr"))]
    #[cfg(not(feature = "ssr"))]
    pub trait Attributes {}
}

#[cfg(feature = "experimental")]
pub mod experimental;
