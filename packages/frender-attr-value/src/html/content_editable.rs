use frender_common::Empty;

use crate::{known::KnownStr, AttrValue, AttrValueKind};

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

impl AttrValue<AttrKindOfContentEditable> for Empty {}
impl AttrValue<AttrKindOfContentEditable> for bool {}
impl<S: ?Sized + KnownStr> AttrValue<AttrKindOfContentEditable> for S {}

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
