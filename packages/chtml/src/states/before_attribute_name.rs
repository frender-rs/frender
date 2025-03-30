//! https://html.spec.whatwg.org/#before-attribute-name-state

use crate::input_stream::{
    BufferedNonAsciiWsPreprocessedInputStream, BufferedPreprocessedInputStream, NonAsciiWsCharacter,
};

use super::{after_attribute_name::AfterAttributeName, attribute_name::AttributeName};
pub struct BeforeAttributeName<'a>(pub(super) BufferedNonAsciiWsPreprocessedInputStream<'a>);

impl<'a> BeforeAttributeName<'a> {
    pub(crate) const fn from_str(s: &'a str) -> Self {
        Self(BufferedNonAsciiWsPreprocessedInputStream::from_str(s))
    }
    pub(crate) const fn into_next_non_trivial_state(self) -> NextNonTrivial<'a> {
        let s = self.0;
        match s.get_next() {
            // U+002F SOLIDUS (/)
            // U+003E GREATER-THAN SIGN (>)
            // EOF
            Some(NonAsciiWsCharacter::SOLIDUS | NonAsciiWsCharacter::GREATER_THAN_SIGN) | None => {
                // Reconsume in the after attribute name state.
                NextNonTrivial::AfterAttributeName(AfterAttributeName::new(s))
            }
            Some(NonAsciiWsCharacter::EQUALS_SIGN) => {
                panic!("unexpected-equals-sign-before-attribute-name parse error")
            }
            _ => NextNonTrivial::AttributeName(AttributeName::new(s.into_ascii_ws())),
        }
    }
}

pub enum NextNonTrivial<'a> {
    AfterAttributeName(AfterAttributeName<'a>),
    AttributeName(AttributeName<'a>),
}
