use async_str_iter::AsyncStrIterator;

use crate::assert::{HtmlAttributeEqValueOrEmpty, SpaceAndHtmlAttributeName};

async_str_iter::Strings!(
    enum SpaceAndHtmlAttributeState {}
    pub struct SpaceAndHtmlAttribute<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty>(
        name!(N),
        eq_value!(V),
    );
);

#[allow(non_snake_case)]
pub fn SpaceAndHtmlAttribute<N: SpaceAndHtmlAttributeName, V: HtmlAttributeEqValueOrEmpty>(
    name: N,
    eq_value: V,
) -> SpaceAndHtmlAttribute<N, V> {
    SpaceAndHtmlAttribute {
        _state: SpaceAndHtmlAttributeState(),
        name,
        eq_value,
    }
}

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

    pub const fn as_inner_str(&self) -> &'a str {
        self.v
    }
}
