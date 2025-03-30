pub mod quoted {
    use crate::parser::{
        input_stream::{BufferedPreprocessedInputStream, Character},
        states::{before_attribute_name::BeforeAttributeName, Data, SelfClosingStartTag},
    };

    pub struct AfterAttributeValueQuoted<'a>(BufferedPreprocessedInputStream<'a>);

    impl<'a> AfterAttributeValueQuoted<'a> {
        pub(crate) const fn new(s: BufferedPreprocessedInputStream<'a>) -> Self {
            Self(s)
        }

        pub(crate) const fn as_stream(&self) -> &BufferedPreprocessedInputStream<'a> {
            &self.0
        }

        pub(crate) const fn into_next_non_trivial(self) -> NextNonTrivial<'a> {
            let (cur, rest) = self.0.consume_the_next_input_character();

            let cur = match cur {
                Some(out) => Some(out.to_character()),
                None => None,
            };
            match cur {
                Some(ch) if ch.is_ascii_whitespace() => NextNonTrivial::BeforeAttributeName(
                    BeforeAttributeName(rest.into_stream().into_non_ascii_ws()),
                ),
                Some(Character::SOLIDUS) => {
                    NextNonTrivial::SelfClosingStartTag(SelfClosingStartTag(rest.into_stream()))
                }
                Some(Character::GREATER_THAN_SIGN) => NextNonTrivial::DataAndEmitCurrentTagToken {
                    state: Data(rest.into_stream()),
                },
                None => NextNonTrivial::EOF,
                Some(_) => {
                    // Anything else
                    panic!("missing-whitespace-between-attributes parse error")
                }
            }
        }
    }

    pub enum NextNonTrivial<'a> {
        BeforeAttributeName(BeforeAttributeName<'a>),
        SelfClosingStartTag(SelfClosingStartTag<'a>),
        DataAndEmitCurrentTagToken {
            state: Data<'a>,
        },
        /// This is an eof-in-tag parse error. Emit an end-of-file token.
        EOF,
    }
}
