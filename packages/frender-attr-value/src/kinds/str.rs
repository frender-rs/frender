use crate::{known::KnownStr, AttrValue, AttrValueKind};

pub enum AttrKindOfStr {}

impl AttrValueKind for AttrKindOfStr {
    type AttrValue<'a> = &'a str;
}

impl<T: ?Sized + KnownStr> AttrValue<AttrKindOfStr> for T {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
