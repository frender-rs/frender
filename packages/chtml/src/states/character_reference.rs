// pub struct CharacterReferenceReturnToAttributeValueDoubleQuoted<'a>(&'a ());
// pub struct CharacterReferenceReturnToAttributeValueSingleQuoted<'a>(&'a ());
// pub struct CharacterReferenceReturnToAttributeValueUnquoted<'a>(&'a ());

use const_array_vec::ArrayVec;
use numeric::{NumericCharacterReference, NumericCharacterReferenceReturn};

use crate::{
    input_stream::{BufferedPreprocessedInputStream, Character, PreprocessOutput},
    leading::cow_str::LeadingCowStr,
};

use self::named::NamedCharacterReference;

use super::attribute_value::AttributeValue;

pub struct CharacterReference<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> CharacterReference<'a> {
    pub(crate) const fn new(s: BufferedPreprocessedInputStream<'a>) -> Self {
        Self(s)
    }

    pub(crate) const fn into_next_non_trivial_consumed_as_part_of_an_attribute(
        self,
    ) -> CharacterReferenceReturn<'a> {
        match self.into_next_state(temporary_buffer::Unknown) {
            Next::Named(state) => match state.into_next_consumed_as_part_of_an_attribute() {
                named::Next::FlushAndReturn {
                    flush_temporary_buffer,
                    rest,
                } => CharacterReferenceReturn {
                    flush_temporary_buffer:
                        CharacterReferenceReturnTemporaryBuffer::NamedCharacters(
                            flush_temporary_buffer,
                        ),
                    input_stream_or_rest: rest,
                },
                named::Next::FlushAndSwitchToAmbiguousAmpersand {
                    flush_temporary_buffer,
                    state,
                } => {
                    _ = ForceMove((flush_temporary_buffer, state));
                    panic!("This crate forbids ambiguous ampersand or undocumented named character references or named character references not ending with semicolon");
                }
            },
            Next::Numeric {
                temporary_buffer,
                state,
            } => {
                let NumericCharacterReferenceReturn {
                    flush_temporary_buffer,
                    rest,
                } = state.into_next_non_trivial(temporary_buffer);

                let flush_temporary_buffer =
                    CharacterReferenceReturnTemporaryBuffer::NumericCharacter(
                        flush_temporary_buffer,
                    );

                CharacterReferenceReturn {
                    flush_temporary_buffer,
                    input_stream_or_rest: rest,
                }
            }
            Next::FlushAndReturn {
                flush_temporary_buffer,
                input_stream,
            } => CharacterReferenceReturn {
                flush_temporary_buffer: {
                    ForceMove(temporary_buffer::Ampersand) = ForceMove(flush_temporary_buffer);
                    CharacterReferenceReturnTemporaryBuffer::Ampersand
                },
                input_stream_or_rest: input_stream,
            },
        }
    }

    const fn into_next_state(self, temporary_buffer: temporary_buffer::Unknown) -> Next<'a> {
        let temporary_buffer = temporary_buffer
            .set(temporary_buffer::EmptyString)
            .into_append_ampersand();
        let (cur, s) = self.0.consume_the_next_input_character();
        match cur {
            Some(ch) if ch.is_ascii_alphanumeric() => Next::Named(NamedCharacterReference {
                temporary_buffer,
                input_stream: s.reconsume(),
            }),
            Some(PreprocessOutput::Character(Character::NUMBER_SIGN)) => {
                let temporary_buffer = temporary_buffer.into_append_number_sign();

                Next::Numeric {
                    temporary_buffer,
                    state: NumericCharacterReference(s.into_stream()),
                }
            }
            _ => Next::FlushAndReturn {
                flush_temporary_buffer: temporary_buffer,
                input_stream: s.reconsume(),
            },
        }
    }

    pub(crate) const fn into_append_to_attribute_value<
        const CSR: usize,
        const SSR: usize,
        const CAP: usize,
    >(
        self,
        ampersand_and_self: BufferedPreprocessedInputStream<'a>,
        attribute_value: &mut AttributeValue<'a, CSR, SSR>,
        attribute_value_before_ampersand: LeadingCowStr<'a, CAP>,
    ) -> BufferedPreprocessedInputStream<'a> {
        let CharacterReferenceReturn {
            flush_temporary_buffer,
            input_stream_or_rest,
        } = self.into_next_non_trivial_consumed_as_part_of_an_attribute();

        // There is no \r because CharacterReference never consumes it.
        let ampersand_and_rest = ampersand_and_self.to_str_trim_trailing(&input_stream_or_rest);

        match flush_temporary_buffer.into_ampersand_or_chars() {
            Ok(temporary_buffer::Ampersand) => {
                // TO DO: use borrowed str if possible
                attribute_value.push_long_live_leading_cow_str(&attribute_value_before_ampersand);

                assert!(matches!(ampersand_and_rest.as_bytes(), b"&"));
                // csr
                attribute_value.value.push_long_live_str(ampersand_and_rest);
                // ssr
                attribute_value.ssr.push_long_live_str(ampersand_and_rest);
            }
            Err(chars) => {
                attribute_value.push_long_live_leading_cow_str(&attribute_value_before_ampersand);

                // csr
                {
                    let mut slice = ::const_core::slice::Iter::new(chars.as_slice());
                    while let Some(&ch) = slice.next() {
                        attribute_value.value.push(ch);
                    }
                }

                // ssr
                {
                    // TO DO: use borrowed str if possible
                    assert!(matches!(ampersand_and_rest.as_bytes()[0], b'&'));
                    attribute_value.ssr.push_long_live_str(ampersand_and_rest);
                }
            }
        }

        input_stream_or_rest
    }
}

pub struct CharacterReferenceReturn<'a> {
    pub flush_temporary_buffer: CharacterReferenceReturnTemporaryBuffer,
    pub input_stream_or_rest: BufferedPreprocessedInputStream<'a>,
}

pub enum CharacterReferenceReturnTemporaryBuffer {
    Ampersand,
    NumericCharacter(temporary_buffer::NumericCharacter),
    NamedCharacters(temporary_buffer::NamedCharacters),
}

impl CharacterReferenceReturnTemporaryBuffer {
    pub const fn into_ampersand_or_chars(
        self,
    ) -> Result<temporary_buffer::Ampersand, ArrayVec<char, 2>> {
        match self {
            CharacterReferenceReturnTemporaryBuffer::Ampersand => Ok(temporary_buffer::Ampersand),
            CharacterReferenceReturnTemporaryBuffer::NumericCharacter(ch) => Err({
                let mut res = ArrayVec::new();
                res.push(ch.code_point);
                res
            }),
            CharacterReferenceReturnTemporaryBuffer::NamedCharacters(named_characters) => {
                Err(named_characters.into_chars())
            }
        }
    }
}

enum Next<'a> {
    Named(NamedCharacterReference<'a>),
    Numeric {
        temporary_buffer: temporary_buffer::AmpersandWithNumberSign,
        state: NumericCharacterReference<'a>,
    },
    FlushAndReturn {
        flush_temporary_buffer: temporary_buffer::Ampersand,
        input_stream: BufferedPreprocessedInputStream<'a>,
    },
}

mod named {
    use crate::input_stream::BufferedPreprocessedInputStream;

    use super::{ncr, temporary_buffer, ForceMove};

    use self::ambiguous_ampersand::AmbiguousAmpersand;

    pub struct NamedCharacterReference<'a> {
        pub(super) temporary_buffer: temporary_buffer::Ampersand,
        pub(super) input_stream: BufferedPreprocessedInputStream<'a>,
    }

    pub enum Next<'a> {
        FlushAndReturn {
            flush_temporary_buffer: temporary_buffer::NamedCharacters,
            rest: BufferedPreprocessedInputStream<'a>,
        },
        /// Note that in this crate AmbiguousAmpersand has more states than the spec
        /// because [`named characters reference table`](ncr::ALL) in this crate is a subset of the spec.
        FlushAndSwitchToAmbiguousAmpersand {
            flush_temporary_buffer: temporary_buffer::Ampersand,
            state: AmbiguousAmpersand<'a>,
        },
    }

    impl<'a> NamedCharacterReference<'a> {
        pub(crate) const fn into_next_consumed_as_part_of_an_attribute(self) -> Next<'a> {
            let Self {
                temporary_buffer,
                input_stream: mut s,
            } = self;
            // let original = s.const_clone();

            let original = s.const_clone();
            const MAX_NAME_LEN: usize = ncr::ALL.max_name_len();
            let next_n = s.next_n::<MAX_NAME_LEN>();
            let next_n = next_n.as_slice();

            let res = ncr::ALL.trim_start_of_preprocess_outputs(next_n);

            if let Some((trimmed, matched)) = res {
                // Append each character to the temporary buffer when it's consumed.
                let temporary_buffer = (temporary_buffer, matched.name);
                // If there is a match

                if
                //
                // true && // If the character reference was consumed as part of an attribute
                !matches!(matched.name.as_bytes().last().copied(), Some(b';'))
                // and the last character matched is not a U+003B SEMICOLON character (;)
                {
                    unreachable!() // In this crate, all character reference should be ended with SEMICOLON
                } else {
                    // Otherwise
                    // If the last character matched is not a U+003B SEMICOLON character (;)
                    if !matches!(matched.name.as_bytes().last().copied(), Some(b';')) {
                        panic!("missing-semicolon-after-character-reference parse error")
                        // Actually this branch should never run
                        // because in this crate, all character reference should be ended with SEMICOLON
                    }

                    _ = ForceMove(temporary_buffer);
                    let temporary_buffer = temporary_buffer::EmptyString;
                    let temporary_buffer =
                        temporary_buffer.into_append_named_characters(matched.characters);

                    let rest = if trimmed.len() == 0 {
                        // If this match is a full match
                        s
                    } else {
                        const {
                            assert!(
                                matches!(ncr::ALL.all_str_len_eq(), Some(len) if len == MAX_NAME_LEN)
                            )
                        }
                        panic!("currently this branch will never run")
                    };
                    Next::FlushAndReturn {
                        flush_temporary_buffer: temporary_buffer,
                        rest,
                    }
                }
            } else {
                // Otherwise
                let s = original;
                Next::FlushAndSwitchToAmbiguousAmpersand {
                    flush_temporary_buffer: temporary_buffer,
                    state: AmbiguousAmpersand(s),
                }
            }
        }
    }

    mod ambiguous_ampersand {
        use crate::input_stream::BufferedPreprocessedInputStream;

        pub struct AmbiguousAmpersand<'a>(pub(super) BufferedPreprocessedInputStream<'a>);
    }
}

mod numeric {
    use crate::input_stream::{BufferedPreprocessedInputStream, Character, PreprocessOutput};

    use super::temporary_buffer::{self, NumericCharacter};

    pub struct NumericCharacterReference<'a>(pub(super) BufferedPreprocessedInputStream<'a>);

    pub struct CharacterLatinLetterX(Character);

    impl CharacterLatinLetterX {
        const fn try_new(ch: Character) -> Option<Self> {
            if matches!(
                ch,
                Character::LATIN_SMALL_LETTER_X | Character::LATIN_CAPITAL_LETTER_X
            ) {
                Some(Self(ch))
            } else {
                None
            }
        }
    }

    impl<'a> NumericCharacterReference<'a> {
        pub(crate) const fn into_next_non_trivial(
            self,
            temporary_buffer: temporary_buffer::AmpersandWithNumberSign,
        ) -> NumericCharacterReferenceReturn<'a> {
            let character_reference_code = 0u32;
            let (append_to_temporary_buffer, end) = match self.into_next() {
                Next::AppendAndSwitchToHexadecimalCharacterReferenceStart {
                    append_to_temporary_buffer,
                    state,
                } => (
                    Some(append_to_temporary_buffer),
                    state
                        .into_next()
                        .into_next_non_trivial(character_reference_code),
                ),
                Next::DecimalCharacterReferenceStart(state) => (
                    None,
                    state
                        .into_next()
                        .into_next_non_trivial(character_reference_code),
                ),
            };

            let temporary_buffer =
                temporary_buffer.into_append_option_latin_letter_x(append_to_temporary_buffer);

            let NumericCharacterReferenceEndNext {
                flush_temporary_buffer,
                rest,
            } = end.into_next(temporary_buffer);

            NumericCharacterReferenceReturn {
                flush_temporary_buffer: NumericCharacter {
                    code_point: flush_temporary_buffer,
                },
                rest,
            }
        }

        const fn into_next(self) -> Next<'a> {
            let (cur, s) = self.0.consume_the_next_input_character();
            let cur = match cur {
                Some(PreprocessOutput::Character(ch)) => CharacterLatinLetterX::try_new(ch),
                _ => None,
            };
            match cur {
                Some(ch) => Next::AppendAndSwitchToHexadecimalCharacterReferenceStart {
                    append_to_temporary_buffer: ch,
                    state: HexadecimalCharacterReferenceStart(s.into_stream()),
                },
                _ => Next::DecimalCharacterReferenceStart(DecimalCharacterReferenceStart(
                    s.reconsume(),
                )),
            }
        }
    }

    pub struct NumericCharacterReferenceReturn<'a> {
        pub flush_temporary_buffer: temporary_buffer::NumericCharacter,
        pub rest: BufferedPreprocessedInputStream<'a>,
    }

    enum Next<'a> {
        AppendAndSwitchToHexadecimalCharacterReferenceStart {
            append_to_temporary_buffer: CharacterLatinLetterX,
            state: HexadecimalCharacterReferenceStart<'a>,
        },
        DecimalCharacterReferenceStart(DecimalCharacterReferenceStart<'a>),
    }

    struct HexadecimalCharacterReferenceStart<'a>(BufferedPreprocessedInputStream<'a>);

    impl<'a> HexadecimalCharacterReferenceStart<'a> {
        const fn into_next(self) -> HexadecimalCharacterReference<'a> {
            let s = self.0;
            match s.get_next() {
                Some(PreprocessOutput::Character(ch)) if ch.to_char().is_ascii_hexdigit() => {
                    HexadecimalCharacterReference(s)
                }
                _ => panic!("absence-of-digits-in-numeric-character-reference parse error"),
            }
        }
    }

    struct HexadecimalCharacterReference<'a>(BufferedPreprocessedInputStream<'a>);

    impl<'a> HexadecimalCharacterReference<'a> {
        const fn into_next_non_trivial(self, mut crc: u32) -> NumericCharacterReferenceEnd<'a> {
            let mut s = self.0;

            loop {
                match s.next() {
                    Some(PreprocessOutput::Character(ch)) if ch.to_char().is_ascii_hexdigit() => {
                        crc = crc
                            .checked_mul(16)
                            .unwrap()
                            .checked_add(
                                (ch.to_char() as u32)
                                    - match ch.to_char() {
                                        '0'..='9' => 0x0030,
                                        'A'..='F' => 0x0037,
                                        'a'..='f' => 0x0057,
                                        _ => unreachable!(),
                                    },
                            )
                            .unwrap();
                    }
                    Some(PreprocessOutput::Character(Character::SEMICOLON)) => {
                        return NumericCharacterReferenceEnd {
                            character_reference_code: crc,
                            input_stream: s,
                        };
                    }
                    _ => panic!("missing-semicolon-after-character-reference parse error"),
                }
            }
        }
    }

    struct DecimalCharacterReferenceStart<'a>(BufferedPreprocessedInputStream<'a>);

    impl<'a> DecimalCharacterReferenceStart<'a> {
        const fn into_next(self) -> DecimalCharacterReference<'a> {
            match self.0.get_next() {
                Some(PreprocessOutput::Character(ch)) if ch.is_ascii_digit() => {
                    DecimalCharacterReference(self.0)
                }
                _ => {
                    panic!("absence-of-digits-in-numeric-character-reference parse error")
                }
            }
        }
    }

    struct DecimalCharacterReference<'a>(BufferedPreprocessedInputStream<'a>);

    impl<'a> DecimalCharacterReference<'a> {
        const fn into_next_non_trivial(self, mut crc: u32) -> NumericCharacterReferenceEnd<'a> {
            let mut s = self.0;

            loop {
                match s.next() {
                    Some(PreprocessOutput::Character(ch)) if ch.is_ascii_digit() => {
                        crc = crc
                            .checked_mul(10)
                            .unwrap()
                            .checked_add((ch.to_char() as u32) - 0x0030)
                            .unwrap();
                    }
                    Some(PreprocessOutput::Character(Character::SEMICOLON)) => {
                        return NumericCharacterReferenceEnd {
                            character_reference_code: crc,
                            input_stream: s,
                        };
                    }
                    _ => panic!("missing-semicolon-after-character-reference parse error"),
                }
            }
        }
    }

    struct NumericCharacterReferenceEnd<'a> {
        character_reference_code: u32,
        input_stream: BufferedPreprocessedInputStream<'a>,
    }

    impl<'a> NumericCharacterReferenceEnd<'a> {
        const fn into_next(
            self,
            temporary_buffer: temporary_buffer::AmpersandWithNumberSignWithOptionLatinLetterX,
        ) -> NumericCharacterReferenceEndNext<'a> {
            let code_point =
                super::num_cr::character_reference_code_to_char(self.character_reference_code);
            let temporary_buffer = temporary_buffer.into_unknown().set(code_point);
            NumericCharacterReferenceEndNext {
                flush_temporary_buffer: temporary_buffer,
                rest: self.input_stream,
            }
        }
    }

    struct NumericCharacterReferenceEndNext<'a> {
        flush_temporary_buffer: char,
        rest: BufferedPreprocessedInputStream<'a>,
    }
}

/// [`Named character references`](https://html.spec.whatwg.org/#named-character-references)
mod ncr {
    use const_array_vec::ArrayVec;

    use crate::input_stream::{Character, PreprocessOutput, PreprocessedInputStream};

    #[derive(Clone, Copy)]
    pub struct Characters(ArrayVec<Character, 2>);

    impl Characters {
        pub const fn into_array_vec(self) -> ArrayVec<Character, 2> {
            self.0
        }
    }

    struct CharactersInput<T>(T);

    impl CharactersInput<char> {
        const fn into_characters(self) -> Characters {
            CharactersInput([self.0]).into_characters()
        }
    }

    impl<const N: usize> CharactersInput<[char; N]> {
        const fn into_characters(self) -> Characters {
            let mut chs = ArrayVec::new();

            let mut iter = ::const_core::slice::Iter::new(self.0.as_slice());
            while let Some(&x) = iter.next() {
                chs.push(Character::new(x));
            }

            Characters(chs)
        }
    }

    #[derive(Clone, Copy)]
    pub struct NamedCharacterReference {
        pub name: &'static str,
        pub characters: Characters,
    }

    pub(crate) struct CharacterReferenceArrayNameLenDesc<const N: usize>(
        [NamedCharacterReference; N],
    );

    impl<const N: usize> CharacterReferenceArrayNameLenDesc<N> {
        const fn make(mut unsorted: [NamedCharacterReference; N]) -> Self {
            ::const_core::slice_sort!(&mut unsorted, gt = |a, b| a.name.len() < b.name.len());
            Self(unsorted)
        }

        pub(crate) const fn max_name_len(&self) -> usize {
            let mut max = 0usize;

            let mut iter = ::const_core::slice::Iter::new(&self.0);

            while let Some(item) = iter.next() {
                let item_str_len = item.name.len();
                if item_str_len > max {
                    max = item_str_len
                }
            }

            max
        }

        pub(crate) const fn trim_start_of_preprocess_outputs<'a>(
            &self,
            this: &'a [PreprocessOutput],
        ) -> Option<(&'a [PreprocessOutput], NamedCharacterReference)> {
            let mut iter = ::const_core::slice::Iter::new(&self.0);
            while let Some(item) = iter.next() {
                if let Some(trimmed) = trim_start(this, item.name) {
                    return Some((trimmed, *item));
                }
            }

            None
        }

        pub(crate) const fn all_str_len_eq(&self) -> Option<usize> {
            let mut len = None;

            let mut iter = ::const_core::slice::Iter::new(&self.0);

            while let Some(NamedCharacterReference {
                name,
                characters: _,
            }) = iter.next()
            {
                match len {
                    Some(prev_len) if prev_len != name.len() => return None,
                    Some(_) => {}
                    None => len = Some(name.len()),
                }
            }

            len
        }
    }

    /// Returns `Some(trimmed)` if and only if `this.starts_with(start)`
    const fn trim_start<'a>(
        this: &'a [PreprocessOutput],
        start: &str,
    ) -> Option<&'a [PreprocessOutput]> {
        let mut start = PreprocessedInputStream::from_str(start);

        let mut cur = this;

        while let Some(expected) = start.next() {
            match cur.split_first() {
                Some((first, rest)) => {
                    if PreprocessOutput::char_eq(&expected, first) {
                        cur = rest;
                    } else {
                        return None;
                    }
                }
                None => return None,
            }
        }

        Some(cur)
    }

    macro_rules! ignore_first {
        ($t:tt $($rest:tt)*) => { $($rest)* };
    }

    macro_rules! define_named_character_references {
        (
            // $vis:vis mod $refs:ident; $(,)?
            $vis_all:vis const $ALL:ident: _; $(,)?
            $($NAME:ident($s:expr , $ch:expr)),+ $(,)?
        ) => {
            $(
                pub const $NAME: &str = $s;
            )+

            $vis_all const $ALL: CharacterReferenceArrayNameLenDesc<{[$(ignore_first!({$s}())),+].len()}> = CharacterReferenceArrayNameLenDesc::make([$(
                NamedCharacterReference {
                    name: $s,
                    characters: CharactersInput($ch).into_characters()
                },
            )+]);

            // $(
            //     pub const $NAME: [PreprocessOutput; $ref_array_bytes.len()] =
            //         array_ascii_map_preprocess_output(*$ref_array_bytes);
            // )+

            // $vis mod $refs {
            //     use crate::input_stream::PreprocessOutput;
            //     $(
            //         pub const $NAME: &[PreprocessOutput] = &super::$NAME;
            //     )+
            // }
        };
    }

    define_named_character_references!(
        // pub mod refs;,
        //
        pub const ALL: _;,
        AMP_UPPER("AMP;", '&'),
        AMP_LOWER("amp;", '&'),
    );

    const fn array_ascii_map_preprocess_output<const N: usize>(
        bytes: [u8; N],
    ) -> [PreprocessOutput; N] {
        let mut out = [PreprocessOutput::CrOrCrlf; N];

        let mut i = 0usize;
        while i < N {
            let b = bytes[i];
            out[i] = PreprocessOutput::Character(Character::new(
                char::from_u32(b as _).expect("ascii byte"),
            ));
            i += 1;
        }

        out
    }
}

/// https://html.spec.whatwg.org/#numeric-character-reference-end-state
mod num_cr {
    use crate::input_stream::{pat_control, pat_noncharacter};

    pub(super) const fn character_reference_code_to_char(crc: u32) -> char {
        // If the number is 0x00, then this is a null-character-reference parse error. Set the character reference code to 0xFFFD.

        let Some(crc) = char::from_u32(crc) else {
            panic!("character-reference-outside-unicode-range parse error or surrogate-character-reference parse error")
        };

        const _: () = assert!(('\x0D' as u32) == 0x0D);
        const _: () = assert!(('\u{80}' as u32) == 0x80);

        match crc {
            '\0' => panic!("null-character-reference parse error"),
            pat_noncharacter!() => panic!("noncharacter-character-reference parse error"),
            // 0x0D, or a control that's not ASCII whitespace
            ch @ ('\x0D' | pat_control!()) if !ch.is_ascii_whitespace() => {
                panic!("control-character-reference parse error")
            }
            '\u{80}' => '\u{20AC}',
            '\u{82}' => '\u{201A}',
            '\u{83}' => '\u{0192}',
            '\u{84}' => '\u{201E}',
            '\u{85}' => '\u{2026}',
            '\u{86}' => '\u{2020}',
            '\u{87}' => '\u{2021}',
            '\u{88}' => '\u{02C6}',
            '\u{89}' => '\u{2030}',
            '\u{8A}' => '\u{0160}',
            '\u{8B}' => '\u{2039}',
            '\u{8C}' => '\u{0152}',
            '\u{8E}' => '\u{017D}',
            '\u{91}' => '\u{2018}',
            '\u{92}' => '\u{2019}',
            '\u{93}' => '\u{201C}',
            '\u{94}' => '\u{201D}',
            '\u{95}' => '\u{2022}',
            '\u{96}' => '\u{2013}',
            '\u{97}' => '\u{2014}',
            '\u{98}' => '\u{02DC}',
            '\u{99}' => '\u{2122}',
            '\u{9A}' => '\u{0161}',
            '\u{9B}' => '\u{203A}',
            '\u{9C}' => '\u{0153}',
            '\u{9E}' => '\u{017E}',
            '\u{9F}' => '\u{0178}',
            crc => crc,
        }
    }
}

pub mod temporary_buffer {
    use const_array_vec::ArrayVec;

    use super::numeric::CharacterLatinLetterX;

    pub struct Unknown;

    impl Unknown {
        pub const fn set<T>(self, v: T) -> T {
            v
        }
    }

    pub struct EmptyString;

    impl EmptyString {
        pub(crate) const fn into_append_ampersand(self) -> Ampersand {
            Ampersand
        }

        pub(crate) const fn into_append_named_characters(
            self,
            chs: super::ncr::Characters,
        ) -> NamedCharacters {
            NamedCharacters(chs)
        }
    }

    /// `&`
    pub struct Ampersand;

    impl Ampersand {
        pub(crate) const fn into_append_number_sign(self) -> AmpersandWithNumberSign {
            AmpersandWithNumberSign
        }
    }

    /// `&#`
    pub struct AmpersandWithNumberSign;

    impl AmpersandWithNumberSign {
        pub const fn into_unknown(self) -> Unknown {
            Unknown
        }

        pub const fn into_append_option_latin_letter_x(
            self,
            ch: Option<CharacterLatinLetterX>,
        ) -> AmpersandWithNumberSignWithOptionLatinLetterX {
            AmpersandWithNumberSignWithOptionLatinLetterX(ch)
        }
    }

    pub struct AmpersandWithNumberSignWithOptionLatinLetterX(Option<CharacterLatinLetterX>);

    impl AmpersandWithNumberSignWithOptionLatinLetterX {
        pub const fn into_unknown(self) -> Unknown {
            Unknown
        }
    }

    pub struct NamedCharacters(super::ncr::Characters);

    impl NamedCharacters {
        pub const fn into_chars(self) -> ArrayVec<char, 2> {
            let mut res = ArrayVec::new();

            let chars = self.0.into_array_vec();
            let mut iter = ::const_core::slice::Iter::new(chars.as_slice());
            while let Some(item) = iter.next() {
                res.push(item.to_char());
            }

            res
        }
    }

    pub struct NumericCharacter {
        // The original is also ssr because it doesn't contain `\r`.
        // pub(crate) original: &'a str,
        pub(crate) code_point: char,
    }
}

struct ForceMove<T>(T);
