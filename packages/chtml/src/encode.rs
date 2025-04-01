use const_array_string::ArrayString;

use crate::leading::cow_str::LeadingCowStr;

mod character_reference;
mod encoded;
mod encoders;

pub mod attribute_value;
pub const fn attribute_value<const CAP: usize>(input: &str) -> attribute_value::Output<'_, CAP> {
    attribute_value::Output::encode(input)
}

/// `Intact("")` means there are no more chunks
enum EncodedChunk<'a, E> {
    Intact(&'a str),
    IntactWithEncoded(&'a str, E),
}

#[derive(Debug, Clone, Copy)]
enum ArrayCowStr<'a, const CAP: usize> {
    Borrowed(&'a str),
    Owned(ArrayString<CAP>),
}

impl<'a, const CAP: usize> ArrayCowStr<'a, CAP> {
    const fn into_with_capacity<const NEW_CAP: usize>(self) -> ArrayCowStr<'a, NEW_CAP> {
        match self {
            ArrayCowStr::Borrowed(v) => ArrayCowStr::Borrowed(v),
            ArrayCowStr::Owned(v) => ArrayCowStr::Owned(v.into_with_capacity()),
        }
    }

    const fn as_str(&self) -> &str {
        match self {
            ArrayCowStr::Borrowed(this) => this,
            ArrayCowStr::Owned(this) => this.as_str(),
        }
    }

    /// Returns `None` if `s` doesn't contain the whole string or `CAP` is too small.
    const fn try_from_long_living_leading_cow_str<const L: usize>(
        s: LeadingCowStr<'a, L>,
    ) -> Option<Self> {
        match s {
            LeadingCowStr::Borrowed(s) => Some(Self::Borrowed(s)),
            LeadingCowStr::Owned(s) => {
                let Some(s) = s.try_to_array_string() else {
                    return None;
                };
                if CAP >= s.len() {
                    Some(Self::Owned(s.into_with_capacity()))
                } else {
                    None
                }
            }
        }
    }
}
