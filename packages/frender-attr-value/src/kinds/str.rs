use crate::{known::KnownStr, AttrValue, AttrValueKind, IntoAttrValue};

pub enum AttrKindOfStr {}

impl AttrValueKind for AttrKindOfStr {
    type AttrValue<'a> = &'a str;
}

impl<T: KnownStr> IntoAttrValue<AttrKindOfStr> for T {
    type IntoAttrValue = StrIntoAttrValue<T>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        StrIntoAttrValue(self)
    }
}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

pub struct StrIntoAttrValue<T>(pub(crate) T);
