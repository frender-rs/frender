use frender_common::Empty;
use frender_reactive_value::value_kind::KindOfOwned;

use crate::{values::cached_some::CachedSome, AttrValue, AttrValueKind, IntoAttrValue};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

/// See html attribute [spellcheck](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck).
///
/// ## Types that impl [`AttrValue<AttrKindOfSpellcheck>`]
///
/// - [`bool`]
///
///   `true` is mapped to `"true". `false` is mapped to "false"`.
///
/// - [`Empty`]
///
///   An empty string, which is the same as `true`.
///
pub enum AttrKindOfSpellcheck {}

impl AttrValueKind for AttrKindOfSpellcheck {
    type AttrValue<'a> = bool;
}

type CachedSomeBool = CachedSome<bool, KindOfOwned<bool>>;

impl IntoAttrValue<AttrKindOfSpellcheck> for bool {
    type IntoAttrValue = CachedSomeBool;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        CachedSome::new(self)
    }
}

pub struct EmptyAsSpellcheck;
impl IntoAttrValue<AttrKindOfSpellcheck> for Empty {
    type IntoAttrValue = EmptyAsSpellcheck;

    fn into_attr_value(self) -> Self::IntoAttrValue {
        EmptyAsSpellcheck
    }
}

impl crate::sealed::AttrValue<AttrKindOfSpellcheck> for CachedSomeBool {}
impl AttrValue<AttrKindOfSpellcheck> for CachedSomeBool {}

impl crate::sealed::AttrValue<AttrKindOfSpellcheck> for EmptyAsSpellcheck {}
impl AttrValue<AttrKindOfSpellcheck> for EmptyAsSpellcheck {}
