use crate::{
    input_stream::{BufferedPreprocessedInputStream, Character},
    leading::cow_str::LeadingCowStr,
};

use super::after_attribute_value::quoted::AfterAttributeValueQuoted;

mod quoted;

pub struct AttributeValue<'a, const CSR: usize, const SSR: usize> {
    /// ```txt
    /// original: value" | value' | value | \r\n\r" | &amp;'
    ///
    /// value:    value  | value  | value | \n\n    | &
    /// ```
    pub(crate) value: LeadingCowStr<'a, CSR>,
    /// ```txt
    /// ssr:      value  | value  | value | \n\n    | &amp;
    /// ```
    ///
    /// The [preprocessed](crate::input_stream::PreprocessedInputStream)
    /// string of the original input stream.
    pub(crate) ssr: LeadingCowStr<'a, SSR>,
}

impl<'a, const CSR: usize, const SSR: usize> AttributeValue<'a, CSR, SSR> {
    const fn new() -> Self {
        Self {
            value: LeadingCowStr::new(),
            ssr: LeadingCowStr::new(),
        }
    }

    /// Caller must ensure the string is both valid and same for ssr and csr
    const fn from_long_live_str(v: &'a str) -> Self {
        Self {
            value: LeadingCowStr::Borrowed(v),
            ssr: LeadingCowStr::Borrowed(v),
        }
    }

    const fn push_long_live<const CSR2: usize, const SSR2: usize>(
        &mut self,
        AttributeValue { value, ssr }: &AttributeValue<'a, CSR2, SSR2>,
    ) {
        self.value.push_long_live_leading_cow_str(value);
        self.ssr.push_long_live_leading_cow_str(ssr);
    }

    pub(crate) const fn push_long_live_leading_cow_str<const CAP: usize>(
        &mut self,
        s: &LeadingCowStr<'a, CAP>,
    ) {
        self.value.push_long_live_leading_cow_str(s);
        self.ssr.push_long_live_leading_cow_str(s);
    }
}

pub struct AttributeValueDoubleQuoted<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> AttributeValueDoubleQuoted<'a> {
    pub(crate) const fn new(v: BufferedPreprocessedInputStream<'a>) -> Self {
        Self(v)
    }

    pub const fn into_next_non_trivial<const CSR: usize, const SSR: usize>(
        self,
    ) -> (AttributeValue<'a, CSR, SSR>, AfterAttributeValueQuoted<'a>) {
        quoted::into_next_non_trivial(self.0, Character::QUOTATION_MARK)
    }
}

pub struct AttributeValueSingleQuoted<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> AttributeValueSingleQuoted<'a> {
    pub(crate) const fn new(v: BufferedPreprocessedInputStream<'a>) -> Self {
        Self(v)
    }

    pub const fn into_next_non_trivial<const CSR: usize, const SSR: usize>(
        self,
    ) -> (AttributeValue<'a, CSR, SSR>, AfterAttributeValueQuoted<'a>) {
        quoted::into_next_non_trivial(self.0, Character::APOSTROPHE)
    }
}

pub struct AttributeValueUnquoted<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> AttributeValueUnquoted<'a> {
    pub(crate) const fn new(v: BufferedPreprocessedInputStream<'a>) -> Self {
        Self(v)
    }
}

pub mod unquoted;
