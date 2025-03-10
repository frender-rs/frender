use crate::Style;

impl<T: Style> crate::sealed::Style for Option<T> {}
impl<T: Style> Style for Option<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
