use const_core::str::Chars;

use crate::{
    encode::{
        character_reference::{
            self, CharEncodedAsCharacterReference, CharacterReferenceMaybeKnown,
        },
        encoders::{
            DoubleQuotedAttributeValue, SingleQuotedAttributeValue, UnquotedAttributeValue,
        },
    },
    leading::cow_str::LeadingCowStr,
};

use super::EncodedChunk;

struct CharsWithOriginal<'chars, 'a> {
    original: &'a str,
    chars: &'chars mut Chars<'a>,
}

impl<'chars, 'a> CharsWithOriginal<'chars, 'a> {
    const fn new(chars: &'chars mut Chars<'a>) -> Self {
        Self {
            original: chars.as_str(),
            chars,
        }
    }
}

struct StrBeforeChars<'a>(&'a [u8]);

impl<'a> StrBeforeChars<'a> {
    const fn as_str(self) -> &'a str {
        match ::core::str::from_utf8(self.0) {
            Ok(v) => v,
            Err(_) => unreachable!(),
        }
    }
}

impl<'a> CharsWithOriginal<'_, 'a> {
    const fn next(&mut self) -> (StrBeforeChars<'a>, Option<char>) {
        let chars_str = self.chars.as_str();

        let before_chars = {
            let original = self.original.as_bytes();
            let bytes = original.split_at(original.len() - chars_str.len()).0;
            StrBeforeChars(bytes)
        };
        let next = self.chars.next();

        (before_chars, next)
    }
}

pub(crate) struct Encoded<'a, Encoder>(Chars<'a>, Encoder);

macro_rules! unwrap_two {
    (
        {$($imp_0:tt)*}
        {$($imp_1:tt)*}
    ) => {
        $($imp_0)*
        $($imp_1)*
    };
}

macro_rules! impl_encoded_imp {
    (
        type $Alias:ident = each_of![$($Encoder:ty),+ $(,)?];
        $imp:tt
    ) => {$(
        const _: () = {
            unwrap_two! {
                {
                    type $Alias = $Encoder;
                }
                $imp
            }
        };
    )+};
}

macro_rules! impl_encoded {
    (
        type $Alias:ident = each_of![$($Encoder:ty),+ $(,)?];
        $($imp:tt)*
    ) => {
        impl_encoded_imp! {
            type $Alias = each_of![$($Encoder),+];
            {$($imp)*}
        }
    };
}
impl_encoded!(
    type Encoder = each_of![
        DoubleQuotedAttributeValue,
        SingleQuotedAttributeValue,
        UnquotedAttributeValue,
    ];
    impl<'a> Encoded<'a, Encoder> {
        const fn next_chunk(&mut self) -> EncodedChunk<'a, CharEncodedAsCharacterReference> {
            let mut chars = CharsWithOriginal::new(&mut self.0);

            loop {
                let (before_ch, ch) = chars.next();

                let Some(ch) = ch else {
                    return EncodedChunk::Intact(before_ch.as_str());
                };

                let encoded =
                    character_reference::encode_character(ch, self.1.test_should_encode(ch));

                match encoded {
                    None => {
                        // consume this char
                    }
                    Some(encoded) => {
                        return EncodedChunk::IntactWithEncoded(before_ch.as_str(), encoded);
                    }
                }
            }
        }

        pub(crate) const fn collect_leading_cow_str<const CAP: usize>(
            mut self,
        ) -> LeadingCowStr<'a, CAP> {
            let mut res = LeadingCowStr::new();

            loop {
                match self.next_chunk() {
                    EncodedChunk::Intact(intact) if intact.is_empty() => {
                        break;
                    }
                    EncodedChunk::Intact(intact) => res.push_long_live_str(intact),
                    EncodedChunk::IntactWithEncoded(intact, e) => {
                        res.push_long_live_str(intact);

                        match e.to_known_or_lower_x_upper_hex() {
                            CharacterReferenceMaybeKnown::Known(e) => res.push_long_live_str(e),
                            CharacterReferenceMaybeKnown::Unknown(e) => res.push_str(e.as_str()),
                        }
                    }
                }
            }

            res
        }
    }
);

impl<'a, Encoder> Encoded<'a, Encoder> {
    pub(crate) const fn from_str(s: &'a str, encoder: Encoder) -> Self {
        Self(Chars::new(s), encoder)
    }
}
