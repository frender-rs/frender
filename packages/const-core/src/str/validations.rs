//! Operations related to UTF-8 validation.

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

macro_rules! try_option {
    ($e:expr) => {
        match $e {
            ::core::option::Option::Some(x) => x,
            ::core::option::Option::None => return ::core::option::Option::None,
        }
    };
}

/// Reads the next code point out of a byte iterator (assuming a
/// UTF-8-like encoding).
///
/// # Panics
///
/// `bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string
/// or this function panics.
#[inline]
pub(super) const fn next_code_point<'a>(bytes: &mut crate::slice::Iter<'a, u8>) -> Option<u32> {
    // Decode UTF-8
    let x = *try_option!(bytes.next());
    if x < 128 {
        return Some(x as u32);
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    // NOTE: Performance is sensitive to the exact formulation here
    let init = utf8_first_byte(x, 2);
    let y = *bytes.next().expect(
        r#"`bytes` produces an UTF-8-like string,
so the iterator must produce a value here"#,
    );
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        let z = *bytes.next().expect(
            r#"`bytes` produces an UTF-8-like string,
so the iterator must produce a value here."#,
        );
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            let w = *bytes.next().expect(
                r#"`bytes` produces an UTF-8-like string,
so the iterator must produce a value here."#,
            );
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    Some(ch)
}

/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;
