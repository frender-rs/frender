use crate::{
    input_stream::{BufferedPreprocessedInputStream, Character, PreprocessOutput},
    leading::cow_str::LeadingCowStr,
    leading::string::LeadingString,
    states::{
        self, before_attribute_name::BeforeAttributeName, character_reference::CharacterReference,
    },
};

use super::{super::after_attribute_value, AttributeValue, AttributeValueUnquoted};

impl<'a> AttributeValueUnquoted<'a> {
    const fn into_next_state<const CAP: usize>(self) -> Next<'a, CAP> {
        let mut s = self.0;
        let full = s.get_full();

        // None means there is no CrOrCrlf
        // No matter if there is CrOfCrlf, csr and ssr are the same
        let mut attribute_value = None::<LeadingString<CAP>>;

        enum BreakState {
            BeforeAttributeName,
            CharacterReference,
            DataAndEmitCurrentTagToken,
            EOF,
        }

        let (rest, state) = loop {
            let (cur, rest) = s.consume_the_next_input_character();
            struct Continue<T>(T);
            Continue(s) = match cur {
                Some(out) if out.to_character().is_ascii_whitespace() => {
                    break (rest, BreakState::BeforeAttributeName)
                }
                Some(PreprocessOutput::Character(Character::AMPERSAND)) => {
                    break (rest, BreakState::CharacterReference)
                }
                Some(PreprocessOutput::Character(Character::GREATER_THAN_SIGN)) => {
                    break (rest, BreakState::DataAndEmitCurrentTagToken)
                }
                Some(PreprocessOutput::Character(Character::NULL)) => {
                    panic!("unexpected-null-character parse error")
                }
                Some(PreprocessOutput::Character(
                    Character::QUOTATION_MARK
                    | Character::APOSTROPHE
                    | Character::LESS_THAN_SIGN
                    | Character::EQUALS_SIGN
                    | Character::GRAVE_ACCENT,
                )) => panic!("unexpected-character-in-unquoted-attribute-value parse error"),
                None => break (rest, BreakState::EOF),
                // Anything else
                Some(out) => {
                    match &mut attribute_value {
                        Some(attribute_value) => attribute_value.push(out.to_char()),
                        None => {
                            if out.is_cr_or_crlf() {
                                let mut v = LeadingString::new();

                                let before =
                                    full.to_str_trim_trailing(&rest.clone_reconsumed().get_full());
                                v.push_str(before);

                                v.push('\n');

                                attribute_value = Some(v);
                            } else {
                                // keep attribute_value as None
                            }
                        }
                    }

                    Continue(rest.into_stream())
                }
            }
        };

        Next {
            attribute_value: {
                let reconsumed = rest.clone_reconsumed();
                match attribute_value {
                    Some(v) => LeadingCowStr::Owned(v),
                    None => {
                        let original = full.to_str_trim_trailing(&reconsumed.get_full());
                        LeadingCowStr::Borrowed(original)
                    }
                }
            },
            state: match state {
                BreakState::BeforeAttributeName => NextState::BeforeAttributeName(
                    BeforeAttributeName(rest.into_stream().into_non_ascii_ws()),
                ),
                BreakState::CharacterReference => NextState::CharacterReference {
                    ampersand_and_state: rest.clone_reconsumed(),
                    state: CharacterReference::new(rest.into_stream()),
                },
                BreakState::DataAndEmitCurrentTagToken => NextState::DataAndEmitCurrentTagToken {
                    state: states::Data(rest.into_stream()),
                },
                BreakState::EOF => NextState::EOF,
            },
        }
    }

    pub(crate) const fn into_next_non_trivial<const CSR: usize, const SSR: usize>(
        mut self,
    ) -> (AttributeValue<'a, CSR, SSR>, NextNonTrivial<'a>) {
        let mut attribute_value = AttributeValue::new();

        let (new_attribute_value, state) = loop {
            struct Continue<T>(T);

            let Next {
                attribute_value: new_attribute_value,
                state,
            } = self.into_next_state::<SSR>();
            Continue(self) = match state {
                NextState::BeforeAttributeName(state) => {
                    break (
                        new_attribute_value,
                        NextNonTrivial::BeforeAttributeName(state),
                    );
                }
                NextState::DataAndEmitCurrentTagToken { state } => {
                    break (
                        new_attribute_value,
                        NextNonTrivial::DataAndEmitCurrentTagToken { state },
                    )
                }
                NextState::EOF => break (new_attribute_value, NextNonTrivial::EOF),
                NextState::CharacterReference {
                    ampersand_and_state,
                    state,
                } => {
                    let input_stream_or_rest = state.into_append_to_attribute_value(
                        ampersand_and_state,
                        &mut attribute_value,
                        new_attribute_value,
                    );

                    Continue(Self(input_stream_or_rest))
                }
            }
        };

        attribute_value.push_long_live_leading_cow_str(&new_attribute_value);

        (attribute_value, state)
    }
}

enum NextState<'a> {
    BeforeAttributeName(BeforeAttributeName<'a>),
    CharacterReference {
        ampersand_and_state: BufferedPreprocessedInputStream<'a>,
        state: CharacterReference<'a>,
    },
    DataAndEmitCurrentTagToken {
        state: states::Data<'a>,
    },
    /// This is an eof-in-tag parse error. Emit an end-of-file token.
    EOF,
}

struct Next<'a, const CAP: usize> {
    attribute_value: LeadingCowStr<'a, CAP>,
    state: NextState<'a>,
}

pub(crate) enum NextNonTrivial<'a> {
    BeforeAttributeName(BeforeAttributeName<'a>),
    DataAndEmitCurrentTagToken {
        state: states::Data<'a>,
    },
    /// This is an eof-in-tag parse error. Emit an end-of-file token.
    EOF,
}

impl<'a> NextNonTrivial<'a> {
    pub(crate) const fn into_after_attribute_value_quoted_next_non_trivial(
        self,
    ) -> after_attribute_value::quoted::NextNonTrivial<'a> {
        use after_attribute_value::quoted::NextNonTrivial as N;
        match self {
            NextNonTrivial::BeforeAttributeName(state) => N::BeforeAttributeName(state),
            NextNonTrivial::DataAndEmitCurrentTagToken { state } => {
                N::DataAndEmitCurrentTagToken { state }
            }
            NextNonTrivial::EOF => N::EOF,
        }
    }
}
