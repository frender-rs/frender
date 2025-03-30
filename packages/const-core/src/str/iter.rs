//! Iterators for `str` methods.

use ::core::fmt;
use ::core::str::from_utf8;

use crate::slice;

use super::validations::next_code_point;

/// An iterator over the [`char`]s of a string slice.
///
///
/// This struct is created by the [`chars`] method on [`str`].
/// See its documentation for more.
///
/// [`char`]: prim@char
/// [`chars`]: str::chars
#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Chars<'a> {
    iter: slice::Iter<'a, u8>,
}

impl<'a> IntoIterator for Chars<'a> {
    type Item = char;
    type IntoIter = ::core::str::Chars<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_str().chars()
    }
}

impl<'a> Chars<'a> {
    pub const fn new(s: &'a str) -> Self {
        Self {
            iter: slice::Iter::new(s.as_bytes()),
        }
    }

    pub const fn const_clone(&self) -> Self {
        Self {
            iter: self.iter.const_clone(),
        }
    }
}

impl<'a> Chars<'a> {
    #[inline]
    pub const fn next(&mut self) -> Option<char> {
        match next_code_point(&mut self.iter) {
            Some(ch) => Some(char::from_u32(ch).expect(
                r#"`str` invariant says `self.iter` is a valid UTF-8 string and
the resulting `ch` is a valid Unicode Scalar Value."#,
            )),
            None => None,
        }
    }
}

impl fmt::Debug for Chars<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chars(")?;
        f.debug_list().entries(self.clone()).finish()?;
        write!(f, ")")?;
        Ok(())
    }
}

impl<'a> Chars<'a> {
    /// Views the underlying data as a subslice of the original data.
    ///
    /// This has the same lifetime as the original slice, and so the
    /// iterator can continue to be used while this exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut chars = "abc".chars();
    ///
    /// assert_eq!(chars.as_str(), "abc");
    /// chars.next();
    /// assert_eq!(chars.as_str(), "bc");
    /// chars.next();
    /// chars.next();
    /// assert_eq!(chars.as_str(), "");
    /// ```
    #[must_use]
    #[inline]
    pub const fn as_str(&self) -> &'a str {
        match from_utf8(self.iter.as_slice()) {
            Ok(v) => v,
            Err(_) => {
                panic!("`Chars` is only made from a str, which guarantees the iter is valid UTF-8")
            }
        }
    }
}
