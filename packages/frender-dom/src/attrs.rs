use frender_attrs::IntoAttributes;

pub struct Attrs<A: IntoAttributes>(pub A);

#[cfg(feature = "csr")]
#[cfg(feature = "experimental")]
mod csr;

#[cfg(feature = "ssr")]
mod ssr;
