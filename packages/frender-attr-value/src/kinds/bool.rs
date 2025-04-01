use frender_common::Empty;

use crate::{AttrValue, AttrValueKind, IntoAttrValue};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

impl AttrValueKind for bool {
    /// Boolean attributes doesn't have value.
    type AttrValue<'a> = ();
}

pub struct EmptyAsTrue;
impl IntoAttrValue<bool> for Empty {
    type IntoAttrValue = EmptyAsTrue;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        EmptyAsTrue
    }
}

// `bool` is not CachedSome because `false` means `None` (removes the attribute)
pub struct BoolAsAttrValue(bool);
impl IntoAttrValue<bool> for bool {
    type IntoAttrValue = BoolAsAttrValue;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        BoolAsAttrValue(self)
    }
}

impl crate::sealed::AttrValue<bool> for EmptyAsTrue {}
impl AttrValue<bool> for EmptyAsTrue {}
impl crate::sealed::AttrValue<bool> for BoolAsAttrValue {}
impl AttrValue<bool> for BoolAsAttrValue {}
