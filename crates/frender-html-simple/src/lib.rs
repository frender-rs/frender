mod macros;

// mod chain;
// pub use chain::*;

pub use frender_html::component::*; // TODO: remove

#[cfg(feature = "csr")]
pub use frender_csr;

#[cfg(feature = "ssr")]
pub use frender_ssr;

#[doc(hidden)]
pub mod __private {
    pub use frender_html;
    pub use frender_html_common::IntoAsyncWritableAttrs;
}
