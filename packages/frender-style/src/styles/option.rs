use crate::Style;

impl<T: Style> Style for Option<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
