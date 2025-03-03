use frender_common::Empty;

use crate::{AttrValue, AttrValueKind};

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

/// See html attribute [spellcheck](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck).
///
/// ## Types that impl [`AttrValue<Spellcheck>`]
///
/// - [`Spellcheck`]
/// - [`bool`]
///
///   `true` is mapped to `"true". `false` is mapped to "false"`.
///
/// - [`Empty`]
///
///   An empty string, which is the same as `true`.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spellcheck(pub bool);

impl Spellcheck {
    /// empty string or true, which indicates that the element should be, if possible, checked for spelling errors
    pub const EMPTY: Self = Self(true);
}

impl AttrValueKind for Spellcheck {
    type AttrValue<'a> = Spellcheck;
}

impl AttrValue<Spellcheck> for Spellcheck {}
impl AttrValue<Spellcheck> for bool {}
impl AttrValue<Spellcheck> for Empty {}
