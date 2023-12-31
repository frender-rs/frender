use async_str_iter::AsyncStrIterator;

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
pub trait HtmlAttributeEqValueOrEmpty:
    html_attribute_eq_value_or_empty::Sealed + AsyncStrIterator
{
}

mod html_attribute_eq_value_or_empty {
    use async_str_iter::AsyncStrIterator;

    use super::HtmlAttributeEqValueOrEmpty;

    pub trait Sealed {}

    impl Sealed for async_str_iter::never::Never {}
    impl HtmlAttributeEqValueOrEmpty for async_str_iter::never::Never {}

    impl Sealed for async_str_iter::empty::Empty {}
    impl HtmlAttributeEqValueOrEmpty for async_str_iter::empty::Empty {}

    impl<V: AsyncStrIterator> Sealed for crate::attr_value::AttrEqValue<V> {}
    impl<V: AsyncStrIterator> HtmlAttributeEqValueOrEmpty for crate::attr_value::AttrEqValue<V> {}

    impl<L: HtmlAttributeEqValueOrEmpty, R: HtmlAttributeEqValueOrEmpty> Sealed
        for async_str_iter::either::IterEither<L, R>
    {
    }
    impl<L: HtmlAttributeEqValueOrEmpty, R: HtmlAttributeEqValueOrEmpty> HtmlAttributeEqValueOrEmpty
        for async_str_iter::either::IterEither<L, R>
    {
    }
}

// Empty or ` a=b c=d`
pub trait SpaceAndHtmlAttributesOrEmpty: Sealed + AsyncStrIterator {}

impl Sealed for async_str_iter::empty::Empty {}
impl SpaceAndHtmlAttributesOrEmpty for async_str_iter::empty::Empty {}

impl<V: AsyncStrIterator> SpaceAndHtmlAttributesOrEmpty for AssertSpaceAndHtmlAttributeName<V> {}

impl<A: SpaceAndHtmlAttributesOrEmpty, B: SpaceAndHtmlAttributesOrEmpty> Sealed
    for async_str_iter::chain::Chain<A, B>
{
}
impl<A: SpaceAndHtmlAttributesOrEmpty, B: SpaceAndHtmlAttributesOrEmpty>
    SpaceAndHtmlAttributesOrEmpty for async_str_iter::chain::Chain<A, B>
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
    for async_str_iter::option::IterOption<crate::attr::SpaceAndHtmlAttribute<N, V>>
{
}
impl<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty> SpaceAndHtmlAttributesOrEmpty
    for async_str_iter::option::IterOption<crate::attr::SpaceAndHtmlAttribute<N, V>>
{
}

/// Any numbers of `<div>...</div>` or `<br>` or `abc` (text).
pub trait HtmlChildren: html_children::Sealed + AsyncStrIterator {}

mod html_children {
    use async_str_iter::AsyncStrIterator;

    use super::HtmlChildren;
    pub trait Sealed {}

    impl Sealed for async_str_iter::empty::Empty {}
    impl HtmlChildren for async_str_iter::empty::Empty {}

    impl<V: AsyncStrIterator> Sealed for crate::encode::Encode<crate::escape_safe::Safe, V> {}
    impl<V: AsyncStrIterator> HtmlChildren for crate::encode::Encode<crate::escape_safe::Safe, V> {}

    impl<T: HtmlChildren> Sealed for async_str_iter::option::IterOption<T> {}
    impl<T: HtmlChildren> HtmlChildren for async_str_iter::option::IterOption<T> {}

    impl<L: HtmlChildren, R: HtmlChildren> Sealed for async_str_iter::either::IterEither<L, R> {}
    impl<L: HtmlChildren, R: HtmlChildren> HtmlChildren for async_str_iter::either::IterEither<L, R> {}

    impl<I: Iterator> Sealed for async_str_iter::flat::Flat<I> where I::Item: HtmlChildren {}
    impl<I: Iterator> HtmlChildren for async_str_iter::flat::Flat<I> where I::Item: HtmlChildren {}

    impl Sealed for crate::scalar::Scalar {}
    impl HtmlChildren for crate::scalar::Scalar {}

    impl<
            T: super::TagName,
            Attrs: super::SpaceAndHtmlAttributesOrEmpty,
            Children: HtmlChildren,
        > Sealed for crate::element::NormalElement<T, Attrs, Children>
    {
    }
    impl<
            T: super::TagName,
            Attrs: super::SpaceAndHtmlAttributesOrEmpty,
            Children: HtmlChildren,
        > HtmlChildren for crate::element::NormalElement<T, Attrs, Children>
    {
    }

    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::ScriptContent> Sealed
        for crate::element::ScriptElement<Attrs, Children>
    {
    }
    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::ScriptContent> HtmlChildren
        for crate::element::ScriptElement<Attrs, Children>
    {
    }

    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::OneStringOrEmpty> Sealed
        for crate::element::StyleElement<Attrs, Children>
    {
    }
    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::OneStringOrEmpty>
        HtmlChildren for crate::element::StyleElement<Attrs, Children>
    {
    }

    impl<T: super::TagName, Attrs: super::SpaceAndHtmlAttributesOrEmpty> Sealed
        for crate::element::VoidElement<T, Attrs>
    {
    }
    impl<T: super::TagName, Attrs: super::SpaceAndHtmlAttributesOrEmpty> HtmlChildren
        for crate::element::VoidElement<T, Attrs>
    {
    }

    macro_rules! impl_for_tuple {
        ($($iter:ident ($($field:ident),+) ,)+) => {$(
                impl<$($field: Sealed + AsyncStrIterator),+> Sealed for async_str_iter::concat::$iter<$($field),+> {}
                impl<$($field: HtmlChildren),+> HtmlChildren for async_str_iter::concat::$iter<$($field),+> {}
        )+};
    }

    impl_for_tuple!(
        IterTuple2(R0, R1),
        IterTuple3(R0, R1, R2),
        IterTuple4(R0, R1, R2, R3),
        IterTuple5(R0, R1, R2, R3, R4),
        IterTuple6(R0, R1, R2, R3, R4, R5),
        IterTuple7(R0, R1, R2, R3, R4, R5, R6),
        IterTuple8(R0, R1, R2, R3, R4, R5, R6, R7),
        IterTuple9(R0, R1, R2, R3, R4, R5, R6, R7, R8),
        IterTuple10(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9),
        IterTuple11(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10),
        IterTuple12(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11),
        IterTuple13(R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12),
    );
}

pub trait OneElement: HtmlChildren {}

mod one_element {
    use super::{HtmlChildren, OneElement};

    impl<
            T: super::TagName,
            Attrs: super::SpaceAndHtmlAttributesOrEmpty,
            Children: HtmlChildren,
        > OneElement for crate::element::NormalElement<T, Attrs, Children>
    {
    }

    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::ScriptContent> OneElement
        for crate::element::ScriptElement<Attrs, Children>
    {
    }

    impl<T: super::TagName, Attrs: super::SpaceAndHtmlAttributesOrEmpty> OneElement
        for crate::element::VoidElement<T, Attrs>
    {
    }

    impl<Attrs: super::SpaceAndHtmlAttributesOrEmpty, Children: super::OneStringOrEmpty> OneElement
        for crate::element::StyleElement<Attrs, Children>
    {
    }
}

pub trait TagName: tag_name::Sealed + AsyncStrIterator {}
mod tag_name {
    use async_str_iter::AsyncStrIterator;

    use super::TagName;
    pub trait Sealed {}

    impl<S: AsyncStrIterator> Sealed for crate::tag::AssertTagName<S> {}
    impl<S: AsyncStrIterator> TagName for crate::tag::AssertTagName<S> {}
}

pub trait ScriptContent: script_content::Sealed + AsyncStrIterator {}

mod script_content {
    use super::ScriptContent;

    pub trait Sealed {}

    impl Sealed for async_str_iter::empty::Empty {}
    impl ScriptContent for async_str_iter::empty::Empty {}

    impl<T: ScriptContent> Sealed for async_str_iter::option::IterOption<T> {}
    impl<T: ScriptContent> ScriptContent for async_str_iter::option::IterOption<T> {}

    impl<L: ScriptContent, R: ScriptContent> Sealed for async_str_iter::either::IterEither<L, R> {}
    impl<L: ScriptContent, R: ScriptContent> ScriptContent
        for async_str_iter::either::IterEither<L, R>
    {
    }

    impl<S: async_str_iter::AsyncStrIterator> Sealed
        for crate::script::IterScriptInnerTextWronglyEncoded<S>
    {
    }
    impl<S: async_str_iter::AsyncStrIterator> ScriptContent
        for crate::script::IterScriptInnerTextWronglyEncoded<S>
    {
    }
}

pub trait OneStringOrEmpty: one_string_or_empty::Sealed + AsyncStrIterator {}

mod one_string_or_empty {
    use super::OneStringOrEmpty;

    pub trait Sealed {}

    impl Sealed for async_str_iter::empty::Empty {}
    impl OneStringOrEmpty for async_str_iter::empty::Empty {}

    impl Sealed for &str {}
    impl OneStringOrEmpty for &str {}

    impl<S: AsRef<str>> Sealed for async_str_iter::any_str::IterAnyStr<S> {}
    impl<S: AsRef<str>> OneStringOrEmpty for async_str_iter::any_str::IterAnyStr<S> {}

    impl<S: std::borrow::Borrow<str>> Sealed for async_str_iter::borrow_str::IterBorrowStr<S> {}
    impl<S: std::borrow::Borrow<str>> OneStringOrEmpty
        for async_str_iter::borrow_str::IterBorrowStr<S>
    {
    }

    impl<T: OneStringOrEmpty> Sealed for async_str_iter::option::IterOption<T> {}
    impl<T: OneStringOrEmpty> OneStringOrEmpty for async_str_iter::option::IterOption<T> {}

    impl<L: OneStringOrEmpty, R: OneStringOrEmpty> Sealed for async_str_iter::either::IterEither<L, R> {}
    impl<L: OneStringOrEmpty, R: OneStringOrEmpty> OneStringOrEmpty
        for async_str_iter::either::IterEither<L, R>
    {
    }
}

pub trait OneString: OneStringOrEmpty {}

mod one_string {
    use super::OneString;

    impl OneString for &str {}
    impl<S: AsRef<str>> OneString for async_str_iter::any_str::IterAnyStr<S> {}
    impl<L: OneString, R: OneString> OneString for async_str_iter::either::IterEither<L, R> {}
}
