use crate::parser::input_stream::{
    BufferedNonAsciiWsPreprocessedInputStream, BufferedPreprocessedInputStream, Character,
    NonAsciiWsCharacter, PreprocessOutput,
};

use super::{
    after_attribute_name::AfterAttributeName, before_attribute_value::BeforeAttributeValue,
};

pub struct AttributeName<'a>(BufferedPreprocessedInputStream<'a>);

impl<'a> AttributeName<'a> {
    pub(crate) const fn new(s: BufferedPreprocessedInputStream<'a>) -> Self {
        Self(s)
    }

    pub(crate) const fn into_next_non_trivial(self) -> NextNonTrivialPayload<'a> {
        let full = self.0.get_full();
        let (after, state) = self.into_next_non_trivial_state();
        NextNonTrivialPayload {
            attribute_name: full.to_str_trim_trailing(&after.get_full()),
            state,
        }
    }

    const fn into_next_non_trivial_state(
        self,
    ) -> (BufferedPreprocessedInputStream<'a>, NextNonTrivial<'a>) {
        let mut s = self.0;
        loop {
            return match s.get_next() {
                Some(ch) => match ch.to_non_ascii_ws() {
                    None // whitespace
                    | Some(NonAsciiWsCharacter::SOLIDUS | NonAsciiWsCharacter::GREATER_THAN_SIGN) =>
                    {
                        // Reconsume in the after attribute name state.
                        (
                            s.const_clone(),
                            NextNonTrivial::AfterAttributeName(AfterAttributeName::new(
                                s.into_non_ascii_ws(),
                            )),
                        )
                    }
                    Some(NonAsciiWsCharacter::EQUALS_SIGN) => {
                        let after = s.const_clone();

                        let consumed = s.next();
                        debug_assert!(matches!(
                            consumed,
                            Some(PreprocessOutput::Character(Character::EQUALS_SIGN))
                        ));
                        (
                            after,
                            NextNonTrivial::BeforeAttributeValue(BeforeAttributeValue::new(
                                s.into_non_ascii_ws(),
                            )),
                        )
                    }
                    Some(NonAsciiWsCharacter::NULL) => {
                        panic!("unexpected-null-character parse error")
                    }
                    Some(
                        NonAsciiWsCharacter::QUOTATION_MARK
                        | NonAsciiWsCharacter::APOSTROPHE
                        | NonAsciiWsCharacter::LESS_THAN_SIGN,
                    ) => {
                        panic!("unexpected-character-in-attribute-name parse error")
                    }
                    Some(_) => {
                        _ = s.next();
                        continue;
                    }
                },
                // EOF
                None => {
                    // Reconsume in the after attribute name state.
                    (
                        s,
                        NextNonTrivial::AfterAttributeName(AfterAttributeName::new(
                            BufferedNonAsciiWsPreprocessedInputStream::EMPTY,
                        )),
                    )
                }
            };
        }
    }
}

pub(crate) struct NextNonTrivialPayload<'a> {
    pub(crate) attribute_name: &'a str,
    pub(crate) state: NextNonTrivial<'a>,
}

pub(crate) enum NextNonTrivial<'a> {
    AfterAttributeName(AfterAttributeName<'a>),
    BeforeAttributeValue(BeforeAttributeValue<'a>),
}
