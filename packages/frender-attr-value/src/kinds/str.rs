use frender_reactive_value::value_kind::KindOfTempRef;

use crate::{
    known::KnownStr, values::cached_some::CachedSome, AttrValue, AttrValueKind, IntoAttrValue,
};

pub enum AttrKindOfStr {}

impl AttrValueKind for AttrKindOfStr {
    type AttrValue<'a> = &'a str;
}

type CachedSomeStr<T: KnownStr> = CachedSome<T, KindOfTempRef<str>>;

impl<T: KnownStr> IntoAttrValue<AttrKindOfStr> for T {
    type IntoAttrValue = CachedSomeStr<T>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        CachedSome::new(self)
    }
}

impl<T: KnownStr> crate::sealed::AttrValue<AttrKindOfStr> for CachedSomeStr<T> {}
impl<T: KnownStr> AttrValue<AttrKindOfStr> for CachedSomeStr<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
