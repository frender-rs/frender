use crate::{AttrValue, AttrValueKind};

pub struct OptionAttrValue<T>(pub(super) Option<T>);

impl<T: AttrValue<AK>, AK: AttrValueKind> crate::sealed::AttrValue<AK> for OptionAttrValue<T> {}
impl<T: AttrValue<AK>, AK: AttrValueKind> AttrValue<AK> for OptionAttrValue<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
