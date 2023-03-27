use std::{borrow::Cow, convert::identity, pin::Pin};

use frender_core::{IntoStaticStr, StaticStr, StaticText};

use crate::{bytes::CowSlicedBytes, impl_ssr_for_bytes, EscapeSafe};

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

impl<W: crate::AsyncWrite + ?Sized, S: IntoStaticStr, E: EscapeSafe>
    ::frender_core::UpdateRenderState<crate::Ssr<W>> for EscapeStr<S, E>
{
    type State = State<CowSlicedBytes<'static>>;
    fn initialize_render_state(self, ctx: &mut crate::SsrContext<Pin<&mut W>>) -> Self::State {
        crate::element::bytes::UnsafeRawHtmlBytes(self.into_static_escaped_cow())
            .initialize_render_state(ctx)
    }
    fn update_render_state(
        self,
        ctx: &mut crate::SsrContext<Pin<&mut W>>,
        state: ::core::pin::Pin<&mut Self::State>,
    ) {
        crate::element::bytes::UnsafeRawHtmlBytes(self.into_static_escaped_cow())
            .update_render_state(ctx, state)
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
