use crate::{AttrValue, AttrValueKind};

#[derive(Debug, Clone, Copy)]
pub enum EitherAttrValue<A, B> {
    A(A),
    B(B),
}

impl<A: AttrValue<AK>, B: AttrValue<AK>, AK: AttrValueKind> AttrValue<AK>
    for EitherAttrValue<A, B>
{
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

#[cfg(feature = "either")]
mod extern_either;
