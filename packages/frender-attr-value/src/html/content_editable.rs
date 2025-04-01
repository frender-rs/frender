use frender_common::Empty;
use frender_reactive_value::value_kind::{KindOfOwned, KindOfTempRef};

use crate::{
    known::KnownStr, values::cached_some::CachedSome, AttrValue, AttrValueKind, IntoAttrValue,
};

/// ## impl [`AttrValue<ContentEditable>`](crate::AttrValue) for
///
/// - [`Empty`](frender_common::Empty)
///
///   As [documented](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable#value),
///   if the attribute is given without a value, like <label contenteditable>Example Label</label>, its value is treated as an empty string,
///   which is the same as `"true"`.
///
/// - [`bool`]
///
///   `true` is mapped to `"true". `false` is mapped to "false"`.
///
/// - strings
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrKindOfContentEditable {}

impl AttrValueKind for AttrKindOfContentEditable {
    type AttrValue<'a> = &'a str;
}

pub struct EmptyAsContentEditable;
impl IntoAttrValue<AttrKindOfContentEditable> for Empty {
    type IntoAttrValue = EmptyAsContentEditable;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        EmptyAsContentEditable
    }
}

impl crate::sealed::AttrValue<AttrKindOfContentEditable> for EmptyAsContentEditable {}
impl AttrValue<AttrKindOfContentEditable> for EmptyAsContentEditable {}

type CachedSomeBool = CachedSome<bool, KindOfOwned<bool>>;
impl IntoAttrValue<AttrKindOfContentEditable> for bool {
    type IntoAttrValue = CachedSomeBool;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        CachedSome::new(self)
    }
}
impl crate::sealed::AttrValue<AttrKindOfContentEditable> for CachedSomeBool {}
impl AttrValue<AttrKindOfContentEditable> for CachedSomeBool {}

type CachedSomeStr<T: KnownStr> = CachedSome<T, KindOfTempRef<str>>;
impl<T: KnownStr> IntoAttrValue<AttrKindOfContentEditable> for T {
    type IntoAttrValue = CachedSomeStr<T>;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        CachedSome::new(self)
    }
}
impl<T: KnownStr> crate::sealed::AttrValue<AttrKindOfContentEditable> for CachedSomeStr<T> {}
impl<T: KnownStr> AttrValue<AttrKindOfContentEditable> for CachedSomeStr<T> {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
