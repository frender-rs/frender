use std::{borrow::Cow, convert::identity};

use frender_common::write::str::AsyncWritableStr;

use crate::{bytes::CowSlicedBytes, impl_ssr_for_bytes, Element, EscapeSafe, IntoStaticStr};

use super::bytes::State;

pub struct EscapedStrWritingState<S: AsyncWritableStr, E: EscapeSafe>(pub S, pub E);

impl<S: AsyncWritableStr, E: EscapeSafe> Unpin for EscapedStrWritingState<S, E> {}

impl<S: AsyncWritableStr, E: EscapeSafe> crate::RenderState for EscapedStrWritingState<S, E> {
    fn poll_render<W: futures_io::AsyncWrite + ?Sized>(
        self: std::pin::Pin<&mut Self>,
        writer: std::pin::Pin<&mut W>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let this = self.get_mut();
        let mut write = crate::write_attrs::AsyncWriteEncodedStr {
            write: writer,
            encode: crate::EscapeSafeRefMut(&mut this.1),
        };

        this.0.poll_write_str_into(cx, &mut write)
    }
}

pub struct EscapeStr<S: IntoStaticStr, E: EscapeSafe>(pub S, pub E);

impl<S: IntoStaticStr, E: EscapeSafe> EscapeStr<S, E> {
    pub fn into_static_escaped<Out>(
        mut self,
        from_original: impl FnOnce(S::StaticStr) -> Out,
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

    pub fn into_static_escaped_cow(mut self) -> Cow<'static, str> {
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
);

#[cfg(feature = "StaticText")]
impl_ssr_for_bytes!(
    const _: fn(StaticText<S>) -> Cow<'static, str> = |self, generics: __![S: StaticStr]| {
        EscapeStr(self, html_escape::encode_safe).into_static_escaped(Into::into, Cow::Owned)
    };
);
