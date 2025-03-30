use const_array_string::ArrayString;

use crate::{
    leading::cow_str::LeadingCowStr,
    leading::string::LeadingString,
    leading::vec::LeadingVec,
    states::{before_attribute_value::BeforeAttributeValue, Data, SelfClosingStartTag},
};

use super::states::{
    after_attribute_name, after_attribute_value,
    attribute_name::{self, AttributeName},
    before_attribute_name::{self, BeforeAttributeName},
    before_attribute_value,
};

pub const fn parse_str<const ATTRS: usize, const CSR: usize, const SSR: usize>(
    s: &str,
) -> AttributesWithInfo<'_, ATTRS, CSR, SSR, 0> {
    parse_str_with_ssr(s)
}

pub const fn parse_str_with_ssr<
    const ATTRS: usize,
    const CSR: usize,
    const SSR: usize,
    const SSR_STRING_CAP: usize,
>(
    s: &str,
) -> AttributesWithInfo<'_, ATTRS, CSR, SSR, SSR_STRING_CAP> {
    let ParseManyOutput { attributes, next } =
        parse_many::<ATTRS, CSR, SSR, SSR_STRING_CAP>(BeforeAttributeName::from_str(s));

    match next {
        ParseEnd::SelfClosingStartTag(_) => panic!("unexpected '/' when parsing attributes"),
        ParseEnd::DataAndEmitCurrentTagToken { .. } => {
            panic!("unexpected '>' when parsing attributes")
        }
        ParseEnd::EOF => {}
    }

    attributes
}

const fn parse_many<
    const ATTRS: usize,
    const CSR: usize,
    const SSR: usize,
    const SSR_STRING_CAP: usize,
>(
    mut s: BeforeAttributeName<'_>,
) -> ParseManyOutput<'_, ATTRS, CSR, SSR, SSR_STRING_CAP> {
    let mut attributes = AttributesWithInfo::new();
    let end = 'before_attribute_name: loop {
        let mut parse_one_out = parse_one::<CSR, SSR>(s);

        'parse_one_out: loop {
            let end = match parse_one_out {
                Ok(ParseOneOutput { attribute, next }) => {
                    attributes.push(attribute);
                    match next {
                        ParseOneNext::BeforeAttributeName(state) => {
                            s = state;
                            continue 'before_attribute_name;
                        }
                        ParseOneNext::AnotherAttributeName(attribute_name) => {
                            parse_one_out = parse_one_from_name::<CSR, SSR>(attribute_name);
                            continue 'parse_one_out;
                        }
                        ParseOneNext::End(end) => end,
                    }
                }
                Err(end) => end,
            };
            break 'before_attribute_name end;
        }
    };

    ParseManyOutput {
        attributes,
        next: end,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AttributesWithInfo<
    'a,
    const ATTRS: usize,
    const CSR: usize,
    const SSR: usize,
    const SSR_STRING_CAP: usize,
> {
    max_csr_cap: usize,
    max_ssr_cap: usize,
    value_csr_len_total: usize,
    ssr_string: SpaceAndAttributesLeadingString<SSR_STRING_CAP>,
    attributes: LeadingVec<Attribute<'a, CSR, SSR>, ATTRS>,
}

pub struct AttributesForRendering<'a, const ATTRS: usize, const SSR_STRING_CAP: usize> {
    attributes: [(&'a str, &'a str); ATTRS],
    ssr_string: SpaceAndAttributesArrayString<SSR_STRING_CAP>,
}

impl<'a, const ATTRS: usize, const SSR_STRING_CAP: usize>
    AttributesForRendering<'a, ATTRS, SSR_STRING_CAP>
{
    pub const fn attributes(&self) -> &[(&'a str, &'a str); ATTRS] {
        &self.attributes
    }

    /// Note that this method is not optimized because [`ArrayString::as_str`] is not optimized.
    /// Thus this method is only meant for const environments.
    pub const fn ssr_str(&self) -> SpaceAndAttributesStr<'_> {
        self.ssr_string.as_str()
    }
}

/// A [`LeadingString`] that is a [`SpaceAndAttributesStr`].
#[derive(Debug, Clone, Copy)]
struct SpaceAndAttributesLeadingString<const CAP: usize>(LeadingString<CAP>);
impl<const CAP: usize> SpaceAndAttributesLeadingString<CAP> {
    const fn new() -> Self {
        Self(LeadingString::new())
    }
    const fn push_attribute<const CSR: usize, const SSR: usize>(
        &mut self,
        attribute: Attribute<'_, CSR, SSR>,
    ) {
        let s = &mut self.0;
        s.push(' ');
        s.push_str(attribute.name);

        match attribute.eq_value {
            AttributeEqValueSsr::Eq { quote, ssr } => {
                let quote = match quote {
                    Some(Quote::Double) => "\"",
                    Some(Quote::Single) => "'",
                    None => "",
                };

                s.push_str(quote);

                match ssr {
                    LeadingCowStr::Borrowed(ssr) => s.push_str(ssr),
                    LeadingCowStr::Owned(ssr) => s.push_leading_string(&ssr),
                }

                s.push_str(quote);
            }
            AttributeEqValueSsr::Empty => {}
        }
    }

    const fn as_leading_string(&self) -> &LeadingString<CAP> {
        &self.0
    }

    /// Panics if `CAP` is too small.
    const fn to_array_string(&self) -> SpaceAndAttributesArrayString<CAP> {
        SpaceAndAttributesArrayString(self.0.to_array_string())
    }
}

/// An [`ArrayString`] that is a [`SpaceAndAttributesStr`].
struct SpaceAndAttributesArrayString<const CAP: usize>(ArrayString<CAP>);

impl<const CAP: usize> SpaceAndAttributesArrayString<CAP> {
    const fn as_str(&self) -> SpaceAndAttributesStr<'_> {
        SpaceAndAttributesStr(self.0.as_str())
    }
}

/// A string that is zero or many groups of *space and attribute*.
#[derive(Debug, Clone, Copy)]
pub struct SpaceAndAttributesStr<'a>(&'a str);

impl<'a> SpaceAndAttributesStr<'a> {
    pub const fn as_str(&self) -> &'a str {
        self.0
    }
}

impl<'a, const ATTRS: usize, const CSR: usize, const SSR: usize, const SSR_STRING_CAP: usize>
    AttributesWithInfo<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>
{
    const fn push(&mut self, attribute: Attribute<'a, CSR, SSR>) {
        let csr = attribute.value.len();
        if csr > self.max_csr_cap {
            self.max_csr_cap = csr
        }

        let ssr = match attribute.eq_value {
            AttributeEqValueSsr::Eq { quote: _, ssr } => ssr.len(),
            AttributeEqValueSsr::Empty => 0,
        };
        if ssr > self.max_ssr_cap {
            self.max_ssr_cap = ssr
        }

        self.value_csr_len_total += csr;

        self.ssr_string.push_attribute(attribute);

        self.attributes.push(attribute);
    }

    const fn new() -> Self {
        Self {
            max_csr_cap: 0,
            max_ssr_cap: 0,
            value_csr_len_total: 0,
            ssr_string: SpaceAndAttributesLeadingString::new(),
            attributes: LeadingVec::new(),
        }
    }

    pub const fn len(&self) -> usize {
        self.attributes.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    const fn as_slice(&self) -> &[Attribute<'a, CSR, SSR>] {
        self.attributes.as_slice()
    }

    pub const fn max_csr_cap(&self) -> usize {
        self.max_csr_cap
    }

    pub const fn max_ssr_cap(&self) -> usize {
        self.max_ssr_cap
    }

    pub const fn value_csr_len_total(&self) -> usize {
        self.value_csr_len_total
    }

    pub const fn ssr_string_len(&self) -> usize {
        self.ssr_string.as_leading_string().len()
    }

    /// Panics if `SSR_STRING_CAP` is too small.
    pub const fn ssr_string_as_str(&self) -> &str {
        self.ssr_string.as_leading_string().as_str()
    }

    /// Panics if at least one of ATTRS, CSR, SSR is too small
    /// or `CAP` is not exactly [`self.len()`](Self::len).
    pub const fn to_array_of_attributes_borrowed<const CAP: usize>(
        &self,
    ) -> [AttributeBorrowed<'_>; CAP] {
        assert!(CAP == self.len());
        let attributes = self.as_slice();
        assert!(CAP == attributes.len());

        const DUMMY: AttributeBorrowed = AttributeBorrowed {
            name: "",
            value: "",
            value_as_ssr: ValueAsSsr::Empty,
        };
        let mut res = [DUMMY; CAP];

        let mut i = 0;

        while i < CAP {
            res[i] = attributes[i].as_borrowed();
            i += 1;
        }

        res
    }

    /// Panics if at least one of ATTRS, CSR, SSR is too small
    /// or `CAP` is not exactly [`self.len()`](Self::len).
    const fn to_array_of_attribute_pairs<const CAP: usize>(&self) -> [(&str, &str); CAP] {
        assert!(CAP == self.len());
        let attributes = self.as_slice();
        assert!(CAP == attributes.len());

        let mut res = const { [("", ""); CAP] };

        let mut i = 0;

        while i < CAP {
            res[i] = attributes[i].as_pair();
            i += 1;
        }

        res
    }

    /// Panics if ATTRS is not exactly [`self.len()`](Self::len).
    /// Panics if at least one of CSR, SSR, SSR_STRING_CAP is too small.
    pub const fn to_attributes_for_rendering(
        &self,
    ) -> AttributesForRendering<'_, ATTRS, SSR_STRING_CAP> {
        AttributesForRendering {
            attributes: self.to_array_of_attribute_pairs(),
            ssr_string: self.ssr_string.to_array_string(),
        }
    }
}

struct ParseManyOutput<
    'a,
    const ATTRS: usize,
    const CSR: usize,
    const SSR: usize,
    const SSR_STRING_CAP: usize,
> {
    attributes: AttributesWithInfo<'a, ATTRS, CSR, SSR, SSR_STRING_CAP>,
    next: ParseEnd<'a>,
}

const fn parse_one<const CSR: usize, const SSR: usize>(
    s: BeforeAttributeName<'_>,
) -> Result<ParseOneOutput<'_, CSR, SSR>, ParseEnd<'_>> {
    match s.into_next_non_trivial_state() {
        before_attribute_name::NextNonTrivial::AfterAttributeName(state) => {
            Err(match state.into_next_non_trivial_state() {
                after_attribute_name::NextNonTrivial::SelfClosingStartTag(state) => {
                    ParseEnd::SelfClosingStartTag(state)
                }
                after_attribute_name::NextNonTrivial::DataAndEmitCurrentTagToken { state } => {
                    ParseEnd::DataAndEmitCurrentTagToken { state }
                }
                after_attribute_name::NextNonTrivial::EOF => ParseEnd::EOF,
                _ => unreachable!(),
            })
        }
        before_attribute_name::NextNonTrivial::AttributeName(attribute_name) => {
            parse_one_from_name::<CSR, SSR>(attribute_name)
        }
    }
}

const fn parse_one_from_name<const CSR: usize, const SSR: usize>(
    attribute_name: AttributeName<'_>,
) -> Result<ParseOneOutput<'_, CSR, SSR>, ParseEnd<'_>> {
    let (attribute_name, before_attribute_value) = {
        let attribute_name::NextNonTrivialPayload {
            attribute_name,
            state,
        } = attribute_name.into_next_non_trivial();

        enum AttributeNameNext<'a> {
            End(ParseEnd<'a>),
            AnotherAttributeName(AttributeName<'a>),
            BeforeAttributeValue(BeforeAttributeValue<'a>),
        }

        let state = match state {
            attribute_name::NextNonTrivial::AfterAttributeName(state) => {
                match state.into_next_non_trivial_state() {
                    after_attribute_name::NextNonTrivial::SelfClosingStartTag(state) => {
                        AttributeNameNext::End(ParseEnd::SelfClosingStartTag(state))
                    }
                    after_attribute_name::NextNonTrivial::DataAndEmitCurrentTagToken { state } => {
                        AttributeNameNext::End(ParseEnd::DataAndEmitCurrentTagToken { state })
                    }
                    after_attribute_name::NextNonTrivial::EOF => {
                        AttributeNameNext::End(ParseEnd::EOF)
                    }
                    after_attribute_name::NextNonTrivial::BeforeAttributeValue(state) => {
                        AttributeNameNext::BeforeAttributeValue(state)
                    }
                    after_attribute_name::NextNonTrivial::AttributeName(state) => {
                        AttributeNameNext::AnotherAttributeName(state)
                    }
                }
            }
            attribute_name::NextNonTrivial::BeforeAttributeValue(state) => {
                AttributeNameNext::BeforeAttributeValue(state)
            }
        };

        match state {
            AttributeNameNext::End(end) => {
                return Ok(ParseOneOutput {
                    attribute: Attribute::new_empty(attribute_name),
                    next: ParseOneNext::End(end),
                })
            }
            AttributeNameNext::AnotherAttributeName(state) => {
                return Ok(ParseOneOutput {
                    attribute: Attribute::new_empty(attribute_name),
                    next: ParseOneNext::AnotherAttributeName(state),
                })
            }
            AttributeNameNext::BeforeAttributeValue(before_attribute_value) => {
                (attribute_name, before_attribute_value)
            }
        }
    };

    let quote;
    let attribute_value;
    let next;
    match before_attribute_value.into_next_non_trivial_state() {
        before_attribute_value::NextNonTrivial::AttributeValueDoubleQuoted(state) => {
            quote = Some(Quote::Double);
            let ns;
            (attribute_value, ns) = state.into_next_non_trivial::<CSR, SSR>();
            next = ns.into_next_non_trivial();
        }
        before_attribute_value::NextNonTrivial::AttributeValueSingleQuoted(state) => {
            quote = Some(Quote::Single);
            let ns;
            (attribute_value, ns) = state.into_next_non_trivial::<CSR, SSR>();
            next = ns.into_next_non_trivial();
        }
        before_attribute_value::NextNonTrivial::AttributeValueUnquoted(state) => {
            quote = None;
            let ns;
            (attribute_value, ns) = state.into_next_non_trivial::<CSR, SSR>();
            next = ns.into_after_attribute_value_quoted_next_non_trivial();
        }
    }

    Ok(ParseOneOutput {
        attribute: Attribute {
            name: attribute_name,
            value: attribute_value.value,
            eq_value: AttributeEqValueSsr::Eq {
                quote,
                ssr: attribute_value.ssr,
            },
        },
        next: ParseOneNext::from_after_attribute_value_quoted_next_non_trivial(next),
    })
}

/// This has the same shape as [`quoted::NextNonTrivial`].
enum ParseOneNext<'a> {
    BeforeAttributeName(BeforeAttributeName<'a>),
    AnotherAttributeName(AttributeName<'a>),
    End(ParseEnd<'a>),
}

enum ParseEnd<'a> {
    SelfClosingStartTag(SelfClosingStartTag<'a>),
    DataAndEmitCurrentTagToken { state: Data<'a> },
    EOF,
}

impl<'a> ParseOneNext<'a> {
    const fn from_after_attribute_value_quoted_next_non_trivial(
        value: after_attribute_value::quoted::NextNonTrivial<'a>,
    ) -> Self {
        use after_attribute_value::quoted::NextNonTrivial as N;
        match value {
            N::BeforeAttributeName(before_attribute_name) => {
                Self::BeforeAttributeName(before_attribute_name)
            }
            N::SelfClosingStartTag(self_closing_start_tag) => {
                Self::End(ParseEnd::SelfClosingStartTag(self_closing_start_tag))
            }
            N::DataAndEmitCurrentTagToken { state } => {
                Self::End(ParseEnd::DataAndEmitCurrentTagToken { state })
            }
            N::EOF => Self::End(ParseEnd::EOF),
        }
    }
}

struct ParseOneOutput<'a, const CSR: usize, const SSR: usize> {
    attribute: Attribute<'a, CSR, SSR>,
    next: ParseOneNext<'a>,
}

#[derive(Debug, Clone, Copy)]
enum AttributeEqValueSsr<'a, const SSR: usize> {
    Eq {
        quote: Option<Quote>,
        ssr: LeadingCowStr<'a, SSR>,
    },
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Attribute<'a, const CSR: usize, const SSR: usize> {
    name: &'a str,
    value: LeadingCowStr<'a, CSR>,
    eq_value: AttributeEqValueSsr<'a, SSR>,
}

#[derive(Debug, Clone, Copy)]
struct AttributeBorrowed<'a> {
    name: &'a str,
    value: &'a str,
    value_as_ssr: ValueAsSsr<'a>,
}

impl<'a> AttributeBorrowed<'a> {
    pub const fn name(&self) -> &'a str {
        self.name
    }

    pub fn value(&self) -> &'a str {
        self.value
    }

    pub fn value_as_ssr(&self) -> ValueAsSsr<'a> {
        self.value_as_ssr
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Quote {
    Double,
    Single,
}

impl<'a, const CSR: usize, const SSR: usize> Attribute<'a, CSR, SSR> {
    const fn new_empty(name: &'a str) -> Self {
        Self {
            name,
            value: LeadingCowStr::new(),
            eq_value: AttributeEqValueSsr::Empty,
        }
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    /// Panics if CSR is too small
    pub const fn value_as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Panics if SSR is too small
    pub const fn value_as_ssr(&self) -> ValueAsSsr<'_> {
        match &self.eq_value {
            AttributeEqValueSsr::Eq { quote, ssr } => ValueAsSsr::Eq {
                quote: *quote,
                ssr: ssr.as_str(),
            },
            AttributeEqValueSsr::Empty => ValueAsSsr::Empty,
        }
    }

    const fn as_pair(&self) -> (&str, &str) {
        (self.name, self.value_as_str())
    }

    const fn as_borrowed(&self) -> AttributeBorrowed<'_> {
        AttributeBorrowed {
            name: self.name,
            value: self.value_as_str(),
            value_as_ssr: self.value_as_ssr(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ValueAsSsr<'a> {
    Empty,
    Eq { quote: Option<Quote>, ssr: &'a str },
}

#[cfg(test)]
mod tests;
