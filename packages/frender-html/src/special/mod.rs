#[cfg(feature = "csr")]
mod parent_only;

pub mod inner_html {
    #[cfg(feature = "csr")]
    pub mod csr;
}
#[cfg(feature = "components")]
pub mod input;
#[cfg(feature = "components")]
pub mod script {
    #[cfg(feature = "csr")]
    pub mod csr;
    #[cfg(feature = "ssr")]
    mod ssr;
}
#[cfg(feature = "components")]
pub mod style {
    #[cfg(feature = "csr")]
    pub mod csr;
    #[cfg(feature = "ssr")]
    mod ssr;
}
#[cfg(feature = "components")]
pub mod textarea {
    #[cfg(feature = "csr")]
    pub mod csr;
    mod props_builder;
    #[cfg(feature = "ssr")]
    pub mod ssr;
}
#[cfg(feature = "components")]
mod void_elements {
    #[cfg(feature = "csr")]
    mod csr;
    #[cfg(feature = "ssr")]
    mod ssr;
}
