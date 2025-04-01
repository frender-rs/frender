use const_array_string::ArrayString;

use crate::leading::cow_str::LeadingCowStr;

use super::{encoded::Encoded, encoders, ArrayCowStr};

#[cfg(feature = "ssr")]
pub mod ssr;

pub struct Output<'a, const CAP: usize> {
    input: &'a str,
    out: EncodedAttributeValueWithAllEncodingsLeading<'a, CAP>,
}

impl<'a, const CAP: usize> Output<'a, CAP> {
    pub const fn min_required_cap_of_eq_value(&self) -> usize {
        self.out.min_required_cap_of_eq_value().0
    }

    pub(super) const fn encode(input: &'a str) -> Self {
        Output {
            input,
            out: if input.is_empty() {
                EncodedAttributeValueWithAllEncodingsLeading::Empty
            } else {
                EncodedAttributeValueWithAllEncodingsLeading::NonEmpty(
                    AttributeValueEncodeKind::encode_all_kinds(input),
                )
            },
        }
    }

    /// Panics if `CAP_SSR_EQ_VALUE` is too small
    pub const fn re_encode_for_rendering<const CAP_SSR_EQ_VALUE: usize>(
        self,
    ) -> AttributeValueForRendering<'a, CAP_SSR_EQ_VALUE> {
        let value = self.input;

        let EncodedAttributeValueLeading {
            kind,
            encoded_attr_value,
        } = self.out.encoded_attr_value_with_min_len_of_eq_value();

        let encoded_attr_value =
            match ArrayCowStr::<CAP_SSR_EQ_VALUE>::try_from_long_living_leading_cow_str(
                encoded_attr_value,
            ) {
                Some(encoded_attr_value) => encoded_attr_value,
                None => ArrayCowStr::try_from_long_living_leading_cow_str(
                    kind.encode::<CAP_SSR_EQ_VALUE>(value),
                )
                .unwrap(),
            };

        let ssr = EncodedAttributeValue {
            kind,
            encoded_attr_value,
        }
        .to_eq_value();

        AttributeValueForRendering { value, ssr }
    }
}

/// Doesn't include `=` or quotes.
#[derive(Debug, Clone, Copy)]
enum EncodedAttributeValueWithAllEncodingsLeading<'a, const CAP: usize> {
    Empty,
    NonEmpty(EncodedNonEmptyAttributeValueWithAllEncodingsLeading<'a, CAP>),
}

impl<'a, const CAP: usize> EncodedAttributeValueWithAllEncodingsLeading<'a, CAP> {
    const fn min_required_cap_of_eq_value(&self) -> (usize, AttributeValueEncodeKind) {
        match self {
            EncodedAttributeValueWithAllEncodingsLeading::Empty => {
                (0, EMPTY_ATTRIBUTE_VALUE_AS_EQ_VALUE_TYPE)
            }
            EncodedAttributeValueWithAllEncodingsLeading::NonEmpty(this) => {
                this.min_len_of_eq_value()
            }
        }
    }

    const fn encoded_attr_value_with_min_len_of_eq_value(
        &self,
    ) -> EncodedAttributeValueLeading<'a, CAP> {
        match self {
            EncodedAttributeValueWithAllEncodingsLeading::Empty => EncodedAttributeValueLeading {
                encoded_attr_value: LeadingCowStr::new(),
                kind: EMPTY_ATTRIBUTE_VALUE_AS_EQ_VALUE_TYPE,
            },
            EncodedAttributeValueWithAllEncodingsLeading::NonEmpty(this) => {
                this.encoded_attr_value_with_min_len_of_eq_value()
            }
        }
    }
}

// Instead of `name`, we choose `name=""` for explicitness
const EMPTY_ATTRIBUTE_VALUE_AS_EQ_VALUE_TYPE: AttributeValueEncodeKind =
    AttributeValueEncodeKind::DoubleQuoted;

#[derive(Debug, Clone, Copy)]
struct EncodedNonEmptyAttributeValueWithAllEncodingsLeading<'a, const CAP: usize> {
    // the strings doesn't include `=` and quotes
    double_quoted: LeadingCowStr<'a, CAP>,
    single_quoted: LeadingCowStr<'a, CAP>,
    unquoted: LeadingCowStr<'a, CAP>,
}

impl<'a, const CAP: usize> EncodedNonEmptyAttributeValueWithAllEncodingsLeading<'a, CAP> {
    /// The min length of `="value"`, `='value'`, `=value`
    const fn min_len_of_eq_value(&self) -> (usize, AttributeValueEncodeKind) {
        let mut all = [
            (
                3 + self.double_quoted.len(),
                AttributeValueEncodeKind::DoubleQuoted,
            ),
            (
                3 + self.single_quoted.len(),
                AttributeValueEncodeKind::SingleQuoted,
            ),
            (1 + self.unquoted.len(), AttributeValueEncodeKind::Unquoted),
        ];
        ::const_core::slice_sort!(&mut all, gt = |a, b| a.0 > b.0);

        all[0]
    }

    const fn encoded_attr_value_with_min_len_of_eq_value(
        &self,
    ) -> EncodedAttributeValueLeading<'a, CAP> {
        let (_, kind) = self.min_len_of_eq_value();

        EncodedAttributeValueLeading {
            encoded_attr_value: match kind {
                AttributeValueEncodeKind::DoubleQuoted => self.double_quoted,
                AttributeValueEncodeKind::SingleQuoted => self.single_quoted,
                AttributeValueEncodeKind::Unquoted => self.unquoted,
            },
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttributeValueEncodeKind {
    DoubleQuoted,
    SingleQuoted,
    Unquoted,
}

macro_rules! impl_encode_kind {
    ($(
        $Kind:ident {
            $field:ident: $encoder:expr
        }
    ),+ $(,)?) => {
        impl AttributeValueEncodeKind {
            const fn encode<const CAP: usize>(
                self,
                input: &str,
            ) -> LeadingCowStr<'_, CAP> {
                match self {$(
                    Self::$Kind => Encoded::from_str(input, $encoder)
                        .collect_leading_cow_str(),
                )+}
            }

            const fn encode_all_kinds<const CAP: usize>(
                input: &str,
            ) -> EncodedNonEmptyAttributeValueWithAllEncodingsLeading<'_, CAP> {
                EncodedNonEmptyAttributeValueWithAllEncodingsLeading {$(
                    $field: Encoded::from_str(input, $encoder)
                        .collect_leading_cow_str(),
                )+}
            }
        }
    };
}

impl_encode_kind!(
    DoubleQuoted {
        double_quoted: encoders::DoubleQuotedAttributeValue
    },
    SingleQuoted {
        single_quoted: encoders::SingleQuotedAttributeValue
    },
    Unquoted {
        unquoted: encoders::UnquotedAttributeValue
    }
);

#[derive(Debug, Clone, Copy)]
struct EqValue<'a, const CAP: usize>(ArrayCowStr<'a, CAP>);

impl<'a, const CAP: usize> EqValue<'a, CAP> {
    pub const fn as_borrowed(&self) -> EqValueStr<'_> {
        EqValueStr(self.0.as_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttributeValueForRendering<'a, const CAP: usize> {
    value: &'a str,
    ssr: EqValue<'a, CAP>,
}

impl<'a, const CAP: usize> AttributeValueForRendering<'a, CAP> {
    pub const fn as_str(&self) -> &'a str {
        self.value
    }

    const fn as_eq_value(&self) -> &EqValue<'a, CAP> {
        &self.ssr
    }

    pub const fn as_eq_value_str(&self) -> EqValueStr<'_> {
        self.as_eq_value().as_borrowed()
    }
}

/// A string that is safe to be used after an attribute name.
/// This string is `="value"` or `='value'` or `=value`.
///
/// Though currently this crate doesn't create empty `EqValueStr("")`,
/// but this might change in the future.
#[derive(Debug, Clone, Copy)]
pub struct EqValueStr<'a>(&'a str);

impl<'a> EqValueStr<'a> {
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

struct EncodedAttributeValueLeading<'a, const CAP: usize> {
    kind: AttributeValueEncodeKind,
    encoded_attr_value: LeadingCowStr<'a, CAP>,
}

struct EncodedAttributeValue<'a, const CAP: usize> {
    kind: AttributeValueEncodeKind,
    encoded_attr_value: ArrayCowStr<'a, CAP>,
}

impl<'a, const CAP: usize> EncodedAttributeValue<'a, CAP> {
    const fn to_eq_value<const CAP_SSR_EQ_VALUE: usize>(
        &self,
    ) -> EqValue<'static, CAP_SSR_EQ_VALUE> {
        let encoded_attr_value = self.encoded_attr_value.as_str();

        EqValue(if encoded_attr_value.is_empty() {
            ArrayCowStr::Borrowed(match self.kind {
                AttributeValueEncodeKind::DoubleQuoted => "=\"\"",
                AttributeValueEncodeKind::SingleQuoted => "=''",
                AttributeValueEncodeKind::Unquoted => {
                    panic!("empty attribute value cannot be encoded as unquoted")
                }
            })
        } else {
            ArrayCowStr::Owned({
                let mut res = ArrayString::new();

                let quote: &str = match self.kind {
                    AttributeValueEncodeKind::DoubleQuoted => "\"",
                    AttributeValueEncodeKind::SingleQuoted => "'",
                    AttributeValueEncodeKind::Unquoted => "",
                };

                res.push_str("=");
                res.push_str(quote);
                res.push_str(encoded_attr_value);
                res.push_str(quote);

                res
            })
        })
    }
}
