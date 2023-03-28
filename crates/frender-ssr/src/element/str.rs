use std::{borrow::Cow, convert::identity};

use frender_core::{IntoStaticStr, StaticStr, StaticText};

use crate::{bytes::CowSlicedBytes, impl_ssr_for_bytes, Element, EscapeSafe};

use super::bytes::State;

pub struct EscapeStr<S: IntoStaticStr, E: EscapeSafe>(pub S, pub E);

impl<S: IntoStaticStr, E: EscapeSafe> EscapeStr<S, E> {
    pub fn into_static_escaped<Out>(
        self,
        from_original: impl FnOnce(S::IntoStaticStr) -> Out,
        from_owned: impl FnOnce(String) -> Out,
    ) -> Out {
        let string = self.0.into_static_str();
        let s = &*string;
        match self.1.escape_safe(s) {
            Cow::Borrowed(b) => {
                if std::ptr::eq(b, s) {
                    from_original(string)
                } else {
                    from_owned(b.to_owned())
                }
            }
            Cow::Owned(s) => from_owned(s),
        }
    }

    pub fn into_static_escaped_cow(self) -> Cow<'static, str> {
        let string = self.0.into_static_str();
        let s = &*string;
        match self.1.escape_safe(s) {
            Cow::Borrowed(b) => {
                if std::ptr::eq(b, s) {
                    string.into()
                } else {
                    Cow::Owned(b.to_owned())
                }
            }
            Cow::Owned(s) => Cow::Owned(s),
        }
    }
}

impl<S: IntoStaticStr, E: EscapeSafe> Element for EscapeStr<S, E> {
    type SsrState = State<CowSlicedBytes<'static>>;
    fn into_ssr_state(self) -> Self::SsrState {
        crate::element::bytes::UnsafeRawHtmlBytes(self.into_static_escaped_cow()).into_ssr_state()
    }
}

impl_ssr_for_bytes!(
    const _: fn(String) -> String =
        |self| EscapeStr(self, html_escape::encode_safe).into_static_escaped(identity, identity);

    const _: fn(&str) -> String =
        |self| EscapeStr(self, html_escape::encode_safe).into_static_escaped(identity, identity);

    const _: fn(Cow<'_, str>) -> Cow<'static, str> = |self| {
        EscapeStr(self, html_escape::encode_safe).into_static_escaped(Cow::Owned, Cow::Owned)
    };

    const _: fn(StaticText<S>) -> Cow<'static, str> = |self, generics: __![S: StaticStr]| {
        EscapeStr(self, html_escape::encode_safe).into_static_escaped(Into::into, Cow::Owned)
    };
);
