use frender_common::Empty;

use crate::{AttrValue, AttrValueKind};

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

impl AttrValue<AttrKindOfSpellcheck> for bool {}
impl AttrValue<AttrKindOfSpellcheck> for Empty {}
