use crate::{
    input_stream::{BufferedPreprocessedInputStream, Character, PreprocessOutput},
    leading::cow_str::LeadingCowStr,
    leading::string::LeadingString,
    states::{
        after_attribute_value::quoted::AfterAttributeValueQuoted,
        character_reference::CharacterReference,
    },
};

use super::AttributeValue;

enum Next<'a, const CAP: usize> {
    AfterAttributeValueQuoted {
        /// Whole decoded
        attribute_value: LeadingCowStr<'a, CAP>,
        /// After the quote
        state: AfterAttributeValueQuoted<'a>,
    },
    CharacterReference {
        /// Decoded attribute value before the '&'
        attribute_value_before_ampersand: LeadingCowStr<'a, CAP>,
        /// '&' + state
        ampersand_and_rest: BufferedPreprocessedInputStream<'a>,
        /// After the '&' (excluding)
        state: CharacterReference<'a>,
    },
}

const fn into_next_state<'a, const CAP: usize>(
    mut s: BufferedPreprocessedInputStream<'a>,
    quote: Character,
) -> Next<'a, CAP> {
    let full = s.get_full();

    // None means there is no CrOrCrlf
    // No matter if there is CrOfCrlf, csr and ssr are the same
    let mut attribute_value = None::<LeadingString<CAP>>;

    struct Continue<T>(T);
    loop {
        let (cur, rest) = s.consume_the_next_input_character();
        Continue(s) = match cur {
            Some(PreprocessOutput::Character(ch)) if ch.to_char() == quote.to_char() => {
                let reconsumed = rest.clone_reconsumed();
                let rest = rest.into_stream();

                let attribute_value = match attribute_value {
                    Some(v) => LeadingCowStr::Owned(v),
                    None => {
                        let original = full.to_str_trim_trailing(&reconsumed.get_full());
                        LeadingCowStr::Borrowed(original)
                    }
                };

                return Next::AfterAttributeValueQuoted {
                    attribute_value,
                    state: AfterAttributeValueQuoted::new(rest),
                };
            }
            Some(PreprocessOutput::Character(Character::AMPERSAND)) => {
                let reconsumed = rest.clone_reconsumed();
                let rest = rest.into_stream();

                let attribute_value_before_ampersand = match attribute_value {
                    Some(v) => LeadingCowStr::Owned(v),
                    None => {
                        LeadingCowStr::Borrowed(full.to_str_trim_trailing(&reconsumed.get_full()))
                    }
                };

                return Next::CharacterReference {
                    attribute_value_before_ampersand,
                    ampersand_and_rest: reconsumed,
                    state: CharacterReference::new(rest),
                };
            }
            Some(PreprocessOutput::Character(Character::NULL)) => {
                panic!("unexpected-null-character parse error")
            }
            // EOF
            None => panic!("eof-in-tag parse error"),
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
        };
    }
}

pub const fn into_next_non_trivial<'a, const CSR: usize, const SSR: usize>(
    mut s: BufferedPreprocessedInputStream<'a>,
    quote: Character,
) -> (AttributeValue<'a, CSR, SSR>, AfterAttributeValueQuoted<'a>) {
    let mut attribute_value = AttributeValue::new();

    loop {
        struct Continue<T>(T);
        Continue(s) = match into_next_state::<SSR>(s, quote) {
            Next::AfterAttributeValueQuoted {
                attribute_value: new_attribute_value,
                state,
            } => {
                attribute_value.push_long_live_leading_cow_str(&new_attribute_value);

                return (attribute_value, state);
            }
            Next::CharacterReference {
                attribute_value_before_ampersand,
                ampersand_and_rest,
                state,
            } => {
                let input_stream_or_rest = state.into_append_to_attribute_value(
                    ampersand_and_rest,
                    &mut attribute_value,
                    attribute_value_before_ampersand,
                );

                Continue(input_stream_or_rest)
            }
        }
    }
}
