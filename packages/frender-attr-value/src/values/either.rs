use crate::{AttrValue, AttrValueKind};

use super::EitherAttrValue;

impl<A: AttrValue<AK>, B: AttrValue<AK>, AK: AttrValueKind> crate::sealed::AttrValue<AK>
    for EitherAttrValue<A, B>
{
}
impl<A: AttrValue<AK>, B: AttrValue<AK>, AK: AttrValueKind> AttrValue<AK>
    for EitherAttrValue<A, B>
{
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
