use crate::Style;

pub enum EitherStyle<A, B> {
    A(A),
    B(B),
}

impl<A: Style, B: Style> Style for EitherStyle<A, B> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;

#[cfg(feature = "either")]
mod extern_either;
