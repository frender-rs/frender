#[cfg(feature = "csr")]
pub mod csr {
    pub use crate::csr::UpdateAttrValue;
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::ssr::SsrAttrValue;
}
