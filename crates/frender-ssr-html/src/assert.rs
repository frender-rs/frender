use frender_common::AsyncStrIterator;

use crate::sealed::Sealed;

pub trait SpaceAndHtmlAttributeName: Sealed + AsyncStrIterator {}

pin_project_lite::pin_project! {
    pub struct AssertSpaceAndHtmlAttributeName<V: AsyncStrIterator> {
        #[pin]
        v: V,
    }
}

impl<V: AsyncStrIterator> AsyncStrIterator for AssertSpaceAndHtmlAttributeName<V> {
    fn poll_next_str(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<&str>> {
        self.project().v.poll_next_str(cx)
    }
}
impl<V: AsyncStrIterator> Sealed for AssertSpaceAndHtmlAttributeName<V> {}
impl<V: AsyncStrIterator> SpaceAndHtmlAttributeName for AssertSpaceAndHtmlAttributeName<V> {}

impl<'a> AssertSpaceAndHtmlAttributeName<&'a str> {
    /// This method is actually stricter than `SpaceAndHtmlAttributeName`.
    /// It only allows ` [\-a-zA-Z]+`.
    pub const fn try_from_str(v: &'a str) -> Option<Self> {
        let bytes = v.as_bytes();

        if bytes.len() <= 1 {
            return None;
        }

        if bytes[0] != b' ' {
            return None;
        }

        let mut i = 1;

        while i < bytes.len() {
            match bytes[i] {
                b'a'..=b'z' | b'A'..=b'Z' | b'-' => i += 1,
                _ => {
                    return None;
                }
            }
        }

        Some(Self { v })
    }

    pub const fn new_from_str(v: &'a str) -> Self {
        if let Some(this) = Self::try_from_str(v) {
            this
        } else {
            panic!("{}", v);
            // panic!("invalid AssertSpaceAndHtmlAttributeName")
        }
    }
}

// pub trait HtmlAttributeName: Sealed + AsyncStrIterator {}

// impl<N: AsyncStrIterator> Sealed
//     for crate::encode::Encode<crate::escape_safe::UnquotedAttribute, N>
// {
// }
// impl<N: AsyncStrIterator> HtmlAttributeName
//     for crate::encode::Encode<crate::escape_safe::UnquotedAttribute, N>
// {
// }

/// `=value` or ` ` (empty)
pub trait HtmlAttributeEqValueOrEmpty: Sealed + AsyncStrIterator {}

impl Sealed for frender_common::async_str::never::Never {}
impl HtmlAttributeEqValueOrEmpty for frender_common::async_str::never::Never {}

impl Sealed for frender_common::async_str::empty::Empty {}
impl HtmlAttributeEqValueOrEmpty for frender_common::async_str::empty::Empty {}

impl<V: AsyncStrIterator> Sealed for crate::attr_value::AttrEqValue<V> {}
impl<V: AsyncStrIterator> HtmlAttributeEqValueOrEmpty for crate::attr_value::AttrEqValue<V> {}

// Empty or ` a=b c=d`
pub trait SpaceAndHtmlAttributesOrEmpty: Sealed + AsyncStrIterator {}

impl SpaceAndHtmlAttributesOrEmpty for frender_common::async_str::empty::Empty {}

impl<V: AsyncStrIterator> SpaceAndHtmlAttributesOrEmpty for AssertSpaceAndHtmlAttributeName<V> {}

impl<A: SpaceAndHtmlAttributesOrEmpty, B: SpaceAndHtmlAttributesOrEmpty> Sealed
    for frender_common::async_str::chain::Chain<A, B>
{
}
impl<A: SpaceAndHtmlAttributesOrEmpty, B: SpaceAndHtmlAttributesOrEmpty>
    SpaceAndHtmlAttributesOrEmpty for frender_common::async_str::chain::Chain<A, B>
{
}

impl<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty> Sealed
    for crate::attr::SpaceAndHtmlAttribute<N, V>
{
}
impl<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty> SpaceAndHtmlAttributesOrEmpty
    for crate::attr::SpaceAndHtmlAttribute<N, V>
{
}

impl<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty> Sealed
    for frender_common::async_str::option::IterOption<crate::attr::SpaceAndHtmlAttribute<N, V>>
{
}
impl<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty> SpaceAndHtmlAttributesOrEmpty
    for frender_common::async_str::option::IterOption<crate::attr::SpaceAndHtmlAttribute<N, V>>
{
}
