use const_array_vec::{ArrayVec, ConstDummyValue};
use const_core::{slice_eq, str::Chars};

mod chars;

/// https://html.spec.whatwg.org/#input-stream
#[derive(Clone)]
pub struct InputStream<'a> {
    chars: Chars<'a>,
}

impl<'a> InputStream<'a> {
    pub const fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}

impl<'a> InputStream<'a> {
    const fn const_clone(&self) -> Self {
        Self {
            chars: self.chars.const_clone(),
        }
    }

    pub(crate) const EMPTY: Self = Self::from_str("");

    const fn from_str(s: &'a str) -> Self {
        Self {
            chars: Chars::new(s),
        }
    }

    const fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    const fn to_str_trim_trailing(&self, trailing: &InputStream<'_>) -> &'a str {
        let this = self.as_str().as_bytes();
        let trailing = trailing.chars.as_str().as_bytes();
        let (ret, rest) = this.split_at(this.len() - trailing.len());

        assert!(
            rest.len() == trailing.len() && {
                let mut eq = true;
                let mut i = 0;
                while eq && i < rest.len() {
                    eq = rest[i] == trailing[i];
                    i += 1;
                }

                eq
            }
        );

        match ::core::str::from_utf8(ret) {
            Ok(v) => v,
            Err(_) => unreachable!(),
        }
    }
}

/// https://html.spec.whatwg.org/#preprocessing-the-input-stream
pub struct PreprocessedInputStream<'a> {
    input_stream: BufferedInputStream<'a>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PreprocessOutput {
    Character(Character),
    CrOrCrlf,
}

impl ConstDummyValue for PreprocessOutput {
    const DUMMY_VALUE: Self = Self::CrOrCrlf;
}

impl PreprocessOutput {
    pub(crate) const fn to_character(self) -> Character {
        match self {
            PreprocessOutput::Character(ch) => ch,
            PreprocessOutput::CrOrCrlf => Character {
                preprocessed: chars::LF,
            },
        }
    }

    pub(crate) const fn to_char(self) -> char {
        self.to_character().to_char()
    }

    pub(crate) const fn to_non_ascii_ws(self) -> Option<NonAsciiWsCharacter> {
        match self {
            PreprocessOutput::Character(ch) => ch.to_non_ascii_ws(),
            PreprocessOutput::CrOrCrlf => None,
        }
    }

    pub(crate) const fn is_ascii_alphanumeric(&self) -> bool {
        self.to_character().is_ascii_alphanumeric()
    }

    pub(crate) const fn is_cr_or_crlf(&self) -> bool {
        match self {
            PreprocessOutput::Character(_) => false,
            PreprocessOutput::CrOrCrlf => true,
        }
    }

    pub(crate) const fn char_eq(&self, other: &Self) -> bool {
        self.to_char() == other.to_char()
    }

    pub(crate) const fn char_ne(&self, other: &Self) -> bool {
        self.to_char() != other.to_char()
    }

    pub(crate) const fn slice_char_eq(a: &[Self], b: &[Self]) -> bool {
        slice_eq!(a, b, ne = |a, b| a.char_ne(b))
    }
}

impl<'a> PreprocessedInputStream<'a> {
    pub(crate) const EMPTY: Self = PreprocessedInputStream {
        input_stream: BufferedInputStream::EMPTY,
    };

    pub(crate) const fn from_str(s: &'a str) -> Self {
        Self {
            input_stream: InputStream::from_str(s).into_buffered(),
        }
    }

    pub const fn next(&mut self) -> Option<PreprocessOutput> {
        match self.try_next() {
            Some(Ok(ch)) => Some(ch),
            // Unlike the spec, this method panics on parse errors for simplicity.
            Some(Err(err)) => err.panic(),
            None => None,
        }
    }

    const fn try_next(&mut self) -> Option<Result<PreprocessOutput, PreprocessParseError>> {
        let Some(ch) = self.input_stream.next() else {
            return None;
        };

        Some(match PreprocessParseErrorInvalidCharKind::test(ch) {
            Ok(ch) => Ok({
                // https://infra.spec.whatwg.org/#normalize-newlines
                if ch == chars::CR {
                    if let Some(ch) = self.input_stream.get_next() {
                        if ch == chars::LF {
                            let expected = self.input_stream.next();
                            debug_assert!(matches!(expected, Some(chars::LF)));
                        }
                    } else {
                    };
                    PreprocessOutput::CrOrCrlf
                } else {
                    PreprocessOutput::Character(Character { preprocessed: ch })
                }
            }),
            Err(kind) => Err(PreprocessParseError {
                kind,
                invalid_char: ch,
            }),
        })
    }

    /// `Some((next_item_and_rest, next_item))`
    const fn next_non_ascii_ws(&mut self) -> Option<(Self, NonAsciiWsCharacter)> {
        loop {
            let next_and_rest = self.const_clone();
            if let Some(ch) = self.next() {
                if let Some(ch) = ch.to_non_ascii_ws() {
                    return Some((next_and_rest, ch));
                }
            } else {
                return None;
            }
        }
    }

    const fn const_clone(&self) -> Self {
        Self {
            input_stream: self.input_stream.const_clone(),
        }
    }

    pub(crate) const fn to_str_trim_trailing(
        &self,
        trailing: &PreprocessedInputStream<'_>,
    ) -> &'a str {
        self.to_str_trim_trailing_option(Some(trailing))
    }

    pub(crate) const fn to_str_trim_trailing_option(
        &self,
        trailing: Option<&PreprocessedInputStream<'_>>,
    ) -> &'a str {
        let trailing = match trailing {
            Some(trailing) => trailing.input_stream.as_full(),
            None => None,
        };
        match trailing {
            Some(trailing) => self.input_stream.get_full().to_str_trim_trailing(trailing),
            None => match self.input_stream.as_full() {
                Some(s) => s.as_str(),
                None => "",
            },
        }
    }
}

struct PreprocessParseError {
    kind: PreprocessParseErrorInvalidCharKind,
    invalid_char: char,
}

impl PreprocessParseError {
    const fn panic<T>(self) -> T {
        let Self {
            kind,
            invalid_char: _,
        } = self;

        match kind {
            PreprocessParseErrorInvalidCharKind::NonCharacter => {
                panic!("noncharacter-in-input-stream")
            }
            PreprocessParseErrorInvalidCharKind::Control => {
                panic!("control-character-in-input-stream")
            }
        }
    }
}

enum PreprocessParseErrorInvalidCharKind {
    // Surrogate, // char is never a surrogate
    NonCharacter,
    Control,
}

/// https://infra.spec.whatwg.org/#noncharacter
macro_rules! pat_noncharacter {
    () => {
        ('\u{FDD0}'..='\u{FDEF}')
            | '\u{FFFE}'
            | '\u{FFFF}'
            | '\u{1FFFE}'
            | '\u{1FFFF}'
            | '\u{2FFFE}'
            | '\u{2FFFF}'
            | '\u{3FFFE}'
            | '\u{3FFFF}'
            | '\u{4FFFE}'
            | '\u{4FFFF}'
            | '\u{5FFFE}'
            | '\u{5FFFF}'
            | '\u{6FFFE}'
            | '\u{6FFFF}'
            | '\u{7FFFE}'
            | '\u{7FFFF}'
            | '\u{8FFFE}'
            | '\u{8FFFF}'
            | '\u{9FFFE}'
            | '\u{9FFFF}'
            | '\u{AFFFE}'
            | '\u{AFFFF}'
            | '\u{BFFFE}'
            | '\u{BFFFF}'
            | '\u{CFFFE}'
            | '\u{CFFFF}'
            | '\u{DFFFE}'
            | '\u{DFFFF}'
            | '\u{EFFFE}'
            | '\u{EFFFF}'
            | '\u{FFFFE}'
            | '\u{FFFFF}'
            | '\u{10FFFE}'
            | '\u{10FFFF}'
    };
}

macro_rules! pat_control {
    () => {
        '\u{007F}'..='\u{009F}'
    };
}

pub(crate) use {pat_control, pat_noncharacter};

impl PreprocessParseErrorInvalidCharKind {
    const fn test(ch: char) -> Result<char, Self> {
        match ch {
            pat_noncharacter!() => Err(Self::NonCharacter),
            char::MIN => Ok(ch),
            // ASCII whitespace is U+0009 TAB, U+000A LF, U+000C FF, U+000D CR, or U+0020 SPACE.
            ch if ch.is_ascii_whitespace() => Ok(ch),
            // A C0 control is a code point in the range U+0000 NULL to U+001F INFORMATION SEPARATOR ONE, inclusive.
            // U+007F DELETE to U+009F APPLICATION PROGRAM COMMAND, inclusive
            pat_control!() => Err(Self::Control),
            ch => Ok(ch),
        }
    }
}

// https://infra.spec.whatwg.org/#normalize-newlines

/// `char` excluding `noncharacters` and `controls`
/// but including `ASCII whitespace` and `U+0000 NULL`
/// and then excluding `U+000D CR`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Character {
    preprocessed: char,
}

impl ConstDummyValue for Character {
    const DUMMY_VALUE: Self = Self::new(char::MIN);
}

impl Character {
    pub(crate) const fn is_ascii_whitespace(&self) -> bool {
        self.preprocessed.is_ascii_whitespace()
    }

    pub(crate) const fn to_non_ascii_ws(self) -> Option<NonAsciiWsCharacter> {
        if self.preprocessed.is_ascii_whitespace() {
            None
        } else {
            Some(NonAsciiWsCharacter(self))
        }
    }

    pub(crate) const fn to_char(self) -> char {
        self.preprocessed
    }

    /// ASCII alphanumeric
    /// U+0030 (0) to U+0039 (9), inclusive
    /// U+0041 (A) to U+005A (Z), inclusive.
    /// U+0061 (a) to U+007A (z), inclusive.
    pub(crate) const fn is_ascii_alphanumeric(&self) -> bool {
        self.to_char().is_ascii_alphanumeric()
    }

    pub(crate) const fn is_ascii_digit(&self) -> bool {
        self.to_char().is_ascii_digit()
    }

    pub(crate) const fn new(ch: char) -> Self {
        match Self::try_new(ch) {
            Some(v) => v,
            None => panic!("invalid Character"),
        }
    }

    pub(crate) const fn try_new(ch: char) -> Option<Self> {
        if ch == chars::CR {
            // '\r' has been preprocessed as '\n'
            return None;
        }
        match PreprocessParseErrorInvalidCharKind::test(ch) {
            Ok(preprocessed) => Some(Self { preprocessed }),
            Err(_) => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NonAsciiWsCharacter(Character);

macro_rules! impl_consts {
    ($($NAME:ident = $ch:expr),+ $(,)?) => {
        impl Character {$(
            pub(crate) const $NAME: Self = Self { preprocessed: $ch };
        )+}
        impl NonAsciiWsCharacter {$(
            pub(crate) const $NAME: Self = Self(Character::$NAME);
        )+}
    };
}

impl_consts!(
    SOLIDUS = '\u{002F}',
    GREATER_THAN_SIGN = '\u{003E}',
    EQUALS_SIGN = '\u{003D}',
    NULL = '\0',
    QUOTATION_MARK = '\u{0022}',
    APOSTROPHE = '\u{0027}',
    LESS_THAN_SIGN = '\u{003C}',
    AMPERSAND = '\u{0026}',
    NUMBER_SIGN = '\u{0023}',
    LATIN_SMALL_LETTER_X = '\u{0078}',
    LATIN_CAPITAL_LETTER_X = '\u{0058}',
    SEMICOLON = '\u{003B}',
    GRAVE_ACCENT = '\u{0060}',
);

impl NonAsciiWsCharacter {
    pub(crate) const fn to_character(self) -> Character {
        self.0
    }

    pub(crate) const fn to_char(self) -> char {
        self.0.to_char()
    }
}

pub struct Buffered<Item, Iter>(Option<BufferedSome<Item, Iter>>);

struct BufferedSome<Item, Iter> {
    /// full = [item, ..rest]
    full: Iter,
    item: Item,
    rest: Iter,
}

impl<Item, Iter> Buffered<Item, Iter> {
    pub(crate) const EMPTY: Self = Self(None);

    pub const fn as_next(&self) -> Option<&Item> {
        match &self.0 {
            Some(BufferedSome { item, .. }) => Some(item),
            None => None,
        }
    }

    pub const fn get_next(&self) -> Option<Item>
    where
        Item: Copy,
    {
        self.as_next().copied()
    }

    pub(crate) const fn as_full(&self) -> Option<&Iter> {
        match &self.0 {
            Some(v) => Some(&v.full),
            None => None,
        }
    }
}

macro_rules! buffered_next {
    ($buffered:expr $(,)?) => {
        buffered_next!($buffered, |iter| {
            let next_and_rest = iter.const_clone();
            match iter.next() {
                ::core::option::Option::Some(next) => Some((next_and_rest, next)),
                ::core::option::Option::None => ::core::option::Option::None,
            }
        })
    };
    ($buffered:expr, |$iter:ident| $next_full_and_first_item:expr $(,)?) => {
        match ::core::option::Option::take(&mut $buffered.0) {
            ::core::option::Option::Some(BufferedSome {
                full: _,
                item,
                rest: mut $iter,
            }) => {
                if let ::core::option::Option::Some((next_and_rest, next)) =
                    $next_full_and_first_item
                {
                    $buffered.0 = ::core::option::Option::Some(BufferedSome {
                        full: next_and_rest,
                        item: next,
                        rest: $iter,
                    })
                }

                ::core::option::Option::Some(item)
            }
            ::core::option::Option::None => ::core::option::Option::None,
        }
    };
}

macro_rules! buffered_const_clone {
    ($buffered:expr $(,)?) => {
        Buffered(match &$buffered.0 {
            ::core::option::Option::Some(BufferedSome { full, item, rest }) => {
                ::core::option::Option::Some(BufferedSome {
                    full: full.const_clone(),
                    item: *item,
                    rest: rest.const_clone(),
                })
            }
            ::core::option::Option::None => ::core::option::Option::None,
        })
    };
}

macro_rules! buffered_get_full {
    ($buffered:expr, $default:expr) => {
        match &$buffered.0 {
            ::core::option::Option::Some(BufferedSome { full, .. }) => full.const_clone(),
            ::core::option::Option::None => $default,
        }
    };
}

macro_rules! buffered_from_unbuffered {
    ($s:expr) => {{
        let mut s = $s;
        let full = s.const_clone();
        Buffered(match s.next() {
            Some(item) => Some(BufferedSome {
                full,
                item,
                rest: s,
            }),
            None => None,
        })
    }};
}

pub type BufferedPreprocessedInputStream<'a> =
    Buffered<PreprocessOutput, PreprocessedInputStream<'a>>;

impl BufferedPreprocessedInputStream<'_> {
    pub const fn next(&mut self) -> Option<PreprocessOutput> {
        buffered_next!(self)
    }

    pub(crate) const fn const_clone(&self) -> Self {
        buffered_const_clone!(self)
    }
}

impl<'a> BufferedPreprocessedInputStream<'a> {
    pub(crate) const fn get_full(&self) -> PreprocessedInputStream<'a> {
        buffered_get_full!(self, PreprocessedInputStream::EMPTY)
    }

    pub(crate) const fn from_str(s: &'a str) -> Self {
        PreprocessedInputStream::from_str(s).into_buffered()
    }

    pub(crate) const fn to_str_trim_trailing(
        &self,
        trailing: &BufferedPreprocessedInputStream<'_>,
    ) -> &'a str {
        self.get_full()
            .to_str_trim_trailing_option(trailing.as_full())
    }

    pub(crate) const fn into_non_ascii_ws(self) -> BufferedNonAsciiWsPreprocessedInputStream<'a> {
        match self.0 {
            Some(BufferedSome {
                full,
                item,
                mut rest,
            }) => Buffered(match item.to_non_ascii_ws() {
                Some(item) => Some(BufferedSome { full, item, rest }),
                None => match rest.next_non_ascii_ws() {
                    Some((full, item)) => Some(BufferedSome { full, item, rest }),
                    None => None,
                },
            }),
            None => Buffered(None),
        }
    }

    pub(crate) const fn next_n<const N: usize>(&mut self) -> ArrayVec<PreprocessOutput, N> {
        let mut res = ArrayVec::new();

        loop {
            if res.is_full() {
                break;
            }
            if let Some(item) = self.next() {
                res.push(item);
            } else {
                break;
            }
        }

        res
    }

    pub(crate) const fn consume_the_next_input_character(
        self,
    ) -> (
        Option<PreprocessOutput>,
        ReconsumablePreprocessedInputStream<'a>,
    ) {
        (self.get_next(), ReconsumablePreprocessedInputStream(self))
    }
}

pub(crate) struct ReconsumablePreprocessedInputStream<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> ReconsumablePreprocessedInputStream<'a> {
    pub(crate) const fn reconsume(self) -> BufferedPreprocessedInputStream<'a> {
        self.0
    }

    pub(crate) const fn clone_reconsumed(&self) -> BufferedPreprocessedInputStream<'a> {
        self.0.const_clone()
    }

    pub(crate) const fn get_current_input_character(&self) -> Option<PreprocessOutput> {
        self.0.get_next()
    }

    pub(crate) const fn as_current_input_character(&self) -> Option<&PreprocessOutput> {
        self.0.as_next()
    }

    pub(crate) const fn into_stream(self) -> BufferedPreprocessedInputStream<'a> {
        let mut s = self.0;
        _ = s.next();
        s
    }
}

pub(crate) type BufferedNonAsciiWsPreprocessedInputStream<'a> =
    Buffered<NonAsciiWsCharacter, PreprocessedInputStream<'a>>;

impl BufferedNonAsciiWsPreprocessedInputStream<'_> {
    pub(crate) const fn next(&mut self) -> Option<NonAsciiWsCharacter> {
        buffered_next!(self, |iter| iter.next_non_ascii_ws())
    }
}
impl<'a> BufferedNonAsciiWsPreprocessedInputStream<'a> {
    pub(crate) const fn from_str(s: &'a str) -> Self {
        BufferedPreprocessedInputStream::from_str(s).into_non_ascii_ws()
    }
    pub(crate) const fn into_ascii_ws(self) -> BufferedPreprocessedInputStream<'a> {
        Buffered(match self.0 {
            Some(BufferedSome { full, item, rest }) => Some(BufferedSome {
                full,
                item: PreprocessOutput::Character(item.to_character()),
                rest,
            }),
            None => None,
        })
    }
}

// pub(crate) struct NonAsciiWsPreprocessedInputStream<'a>(PreprocessedInputStream<'a>);

// impl<'a> NonAsciiWsPreprocessedInputStream<'a> {
//     const fn next(&mut self) -> Option<NonAsciiWsCharacter> {
//         self.0.next_non_ascii_ws()
//     }
// }

type BufferedInputStream<'a> = Buffered<char, InputStream<'a>>;

impl BufferedInputStream<'_> {
    const fn next(&mut self) -> Option<char> {
        buffered_next!(self)
    }
    const fn const_clone(&self) -> Self {
        buffered_const_clone!(self)
    }
}
impl<'a> BufferedInputStream<'a> {
    const fn get_full(&self) -> InputStream<'a> {
        buffered_get_full!(self, InputStream::EMPTY)
    }
}

impl<'a> InputStream<'a> {
    const fn into_buffered(self) -> BufferedInputStream<'a> {
        buffered_from_unbuffered!(self)
    }
}

impl<'a> PreprocessedInputStream<'a> {
    const fn into_buffered(self) -> BufferedPreprocessedInputStream<'a> {
        buffered_from_unbuffered!(self)
    }
}

/*
pub(crate) type BufferedChars<'a> = Buffered<char, Chars<'a>>;

impl<'a> BufferedChars<'a> {
    pub(crate) const fn next(&mut self) -> Option<char> {
        buffered_next!(self)
    }
    pub(crate) const fn const_clone(&self) -> Self {
        buffered_const_clone!(self)
    }
    pub(crate) const fn from_unbuffered(chars: Chars<'a>) -> Self {
        buffered_from_unbuffered!(chars)
    }
    pub(crate) const fn full_as_str(&self) -> &'a str {
        match self.as_full() {
            Some(full) => full.as_str(),
            None => "",
        }
    }
}
*/

#[cfg(test)]
mod tests;
