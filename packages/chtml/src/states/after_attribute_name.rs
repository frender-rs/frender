use crate::input_stream::{BufferedNonAsciiWsPreprocessedInputStream, NonAsciiWsCharacter};

use super::{
    attribute_name::AttributeName, before_attribute_value::BeforeAttributeValue, Data,
    SelfClosingStartTag,
};

pub struct AfterAttributeName<'a>(BufferedNonAsciiWsPreprocessedInputStream<'a>);

impl<'a> AfterAttributeName<'a> {
    pub(crate) const fn new(s: BufferedNonAsciiWsPreprocessedInputStream<'a>) -> Self {
        Self(s)
    }

    pub(crate) const fn into_next_non_trivial_state(self) -> NextNonTrivial<'a> {
        let mut s = self.0;
        match s.get_next() {
            Some(NonAsciiWsCharacter::SOLIDUS) => {
                NextNonTrivial::SelfClosingStartTag(SelfClosingStartTag({
                    _ = s.next();
                    s.into_ascii_ws()
                }))
            }
            Some(NonAsciiWsCharacter::EQUALS_SIGN) => NextNonTrivial::BeforeAttributeValue({
                let consumed = s.next();

                debug_assert!(matches!(consumed, Some(NonAsciiWsCharacter::EQUALS_SIGN)));

                BeforeAttributeValue::new(s)
            }),
            Some(NonAsciiWsCharacter::GREATER_THAN_SIGN) => {
                NextNonTrivial::DataAndEmitCurrentTagToken {
                    state: Data({
                        _ = s.next();
                        s.into_ascii_ws()
                    }),
                }
            }
            None => NextNonTrivial::EOF,
            _ => NextNonTrivial::AttributeName(AttributeName::new(s.into_ascii_ws())),
        }
    }
}

pub(crate) enum NextNonTrivial<'a> {
    SelfClosingStartTag(SelfClosingStartTag<'a>),
    BeforeAttributeValue(BeforeAttributeValue<'a>),
    DataAndEmitCurrentTagToken {
        state: Data<'a>,
    },
    /// This is an eof-in-tag parse error. Emit an end-of-file token.
    EOF,
    AttributeName(AttributeName<'a>),
}
