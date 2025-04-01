use const_array_string::ArrayString;

pub struct CharEncodedAsCharacterReference(char);
const CR_MAX_HEX: usize = {
    let cap = (u32::BITS - (char::MAX as u32).leading_zeros()).div_ceil(
        // One hex contains 4 bits.
        4,
    );

    if const { usize::BITS < u32::BITS } {
        assert!(cap <= (usize::MAX as u32));
    }

    cap as usize
};

const CR_MAX_LEN: usize = "&#x;".len() + CR_MAX_HEX;
pub struct CharacterReferenceLowerHexUpperHex(ArrayString<CR_MAX_LEN>);
impl CharacterReferenceLowerHexUpperHex {
    pub const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub enum CharacterReferenceMaybeKnown {
    Known(&'static str),
    Unknown(CharacterReferenceLowerHexUpperHex),
}

impl CharacterReferenceMaybeKnown {
    pub const fn as_str(&self) -> &str {
        match self {
            CharacterReferenceMaybeKnown::Known(this) => this,
            CharacterReferenceMaybeKnown::Unknown(this) => this.as_str(),
        }
    }
}

// https://doc.rust-lang.org/stable/src/core/fmt/num.rs.html#123
struct UpperHex;
macro_rules! radix {
    ($T:ident, $base:expr, $prefix:expr, $($x:pat => $conv:expr),+) => {
        impl $T {
            const BASE: u8 = $base;
            const PREFIX: &'static str = $prefix;
            const fn digit(x: u8) -> u8 {
                match x {
                    $($x => $conv,)+
                    _ => panic!("number not in the range"),
                }
            }
        }
    }
}
radix! { UpperHex, 16, "0x", x @  0 ..=  9 => b'0' + x, x @ 10 ..= 15 => b'A' + (x - 10) }

impl CharEncodedAsCharacterReference {
    pub const fn to_known_or_lower_x_upper_hex(&self) -> CharacterReferenceMaybeKnown {
        CharacterReferenceMaybeKnown::Known(match self.0 {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&#x27;",
            '\0' => "&#0;",
            _ => {
                return CharacterReferenceMaybeKnown::Unknown(
                    self.to_array_string_lower_x_upper_hex(),
                )
            }
        })
    }
    const fn to_array_string_lower_x_upper_hex(&self) -> CharacterReferenceLowerHexUpperHex {
        let mut x = self.0 as u32;
        // https://doc.rust-lang.org/stable/src/core/fmt/num.rs.html#57
        const ZERO: u32 = 0;
        let mut buf = [0u8; CR_MAX_HEX];
        let mut curr = buf.len();
        const BASE: u32 = UpperHex::BASE as _;
        // Accumulate each digit of the number from the least significant
        // to the most significant figure.
        loop {
            let n = x % BASE; // Get the current place value.
            x = x / BASE; // Deaccumulate the number.
            curr -= 1;
            buf[curr] = UpperHex::digit(n as u8); // Store the digit in the buffer.
            if x == ZERO {
                // No more digits left to accumulate.
                break;
            };
        }

        let buf = buf.split_at(curr).1;
        // SAFETY: The only chars in `buf` are created by `Self::digit` which are assumed to be
        // valid UTF-8
        let Ok(buf) = ::core::str::from_utf8(buf) else {
            unreachable!()
        };

        CharacterReferenceLowerHexUpperHex({
            let mut res = ArrayString::new();
            res.push_str("&#x");
            res.push_str(buf);
            res.push_str(";");
            res
        })
    }
}

const fn html_character_or_encode_character_reference(
    ch: char,
) -> Option<CharEncodedAsCharacterReference> {
    if crate::input_stream::Character::try_new(ch).is_none() {
        Some(CharEncodedAsCharacterReference(ch))
    } else {
        None
    }
}

pub const fn encode_character(ch: char, force: bool) -> Option<CharEncodedAsCharacterReference> {
    if force {
        Some(CharEncodedAsCharacterReference(ch))
    } else {
        // only encode if ch is not a valid Character
        html_character_or_encode_character_reference(ch)
    }
}
