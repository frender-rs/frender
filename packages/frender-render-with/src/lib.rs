#![cfg_attr(feature = "nightly", feature(impl_trait_in_assoc_type))]

#[cfg(feature = "csr")]
pub use self::csr::{
    CsrRenderContext, FnOnceRenderWithContext, IntoFnOnceRenderWithContext, RenderHtml, Rendered,
};

#[cfg(feature = "csr")]
mod csr;

pub struct RenderWith<F>(pub F);
