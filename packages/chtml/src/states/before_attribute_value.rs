use crate::input_stream::{BufferedNonAsciiWsPreprocessedInputStream, NonAsciiWsCharacter};

use super::attribute_value::{
    AttributeValueDoubleQuoted, AttributeValueSingleQuoted, AttributeValueUnquoted,
};

pub struct BeforeAttributeValue<'a>(BufferedNonAsciiWsPreprocessedInputStream<'a>);

impl<'a> BeforeAttributeValue<'a> {
    pub(crate) const fn new(s: BufferedNonAsciiWsPreprocessedInputStream<'a>) -> Self {
        Self(s)
    }

    pub(crate) const fn into_next_non_trivial_state(self) -> NextNonTrivial<'a> {
        let mut s = self.0;
        match s.get_next() {
            Some(NonAsciiWsCharacter::QUOTATION_MARK) => {
                let consumed = s.next();
                debug_assert!(matches!(
                    consumed,
                    Some(NonAsciiWsCharacter::QUOTATION_MARK)
                ));
                NextNonTrivial::AttributeValueDoubleQuoted(AttributeValueDoubleQuoted::new(
                    s.into_ascii_ws(),
                ))
            }
            Some(NonAsciiWsCharacter::APOSTROPHE) => {
                let consumed = s.next();
                debug_assert!(matches!(consumed, Some(NonAsciiWsCharacter::APOSTROPHE)));
                NextNonTrivial::AttributeValueSingleQuoted(AttributeValueSingleQuoted::new(
                    s.into_ascii_ws(),
                ))
            }
            Some(NonAsciiWsCharacter::GREATER_THAN_SIGN) => {
                panic!("missing-attribute-value parse error")
            }
            _ => NextNonTrivial::AttributeValueUnquoted(AttributeValueUnquoted::new(
                s.into_ascii_ws(),
            )),
        }
    }
}

pub(crate) enum NextNonTrivial<'a> {
    AttributeValueDoubleQuoted(AttributeValueDoubleQuoted<'a>),
    AttributeValueSingleQuoted(AttributeValueSingleQuoted<'a>),
    AttributeValueUnquoted(AttributeValueUnquoted<'a>),
}
