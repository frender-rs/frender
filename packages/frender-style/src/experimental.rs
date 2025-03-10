#[cfg(feature = "csr")]
pub mod csr {
    pub use crate::csr::CsrStyle;
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::ssr::{SsrDeclarationList, SsrStyle};
}
