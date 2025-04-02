#[cfg(feature = "csr")]
pub mod csr {
    pub use crate::csr::{CsrAttributes, RenderAttributes};
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub use crate::ssr::SsrAttributes;
}
