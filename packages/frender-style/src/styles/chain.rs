use crate::Style;

#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);

impl<A: Style, B: Style> crate::sealed::Style for Chain<A, B> {}
impl<A: Style, B: Style> Style for Chain<A, B> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
