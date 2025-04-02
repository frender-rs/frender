use frender_attrs::IntoAttributes;

pub struct Attrs<A: IntoAttributes>(pub A);

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
