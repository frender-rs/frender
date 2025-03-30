use crate::{AttrValue, AttrValueKind};

impl<T: AttrValue<AK>, AK: AttrValueKind> crate::sealed::AttrValue<AK> for Option<T> {}
impl<T: AttrValue<AK>, AK: AttrValueKind> AttrValue<AK> for Option<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
