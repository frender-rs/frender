use std::{marker::PhantomData, str::FromStr};

mod imp_wasm {
    pub(super) type StaticOwnedStr = String;
}
mod imp_not_wasm {
    pub(super) type StaticOwnedStr = std::rc::Rc<str>;
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod imp {
    pub(super) type Repr = web_sys::js_sys::JsString;
    pub(super) use Result::Ok as ResultJsString;

    pub(super) use super::imp_wasm::*;
    pub(super) fn into_static_owned_string(repr: Repr) -> StaticOwnedStr {
        repr.into()
    }

    pub(super) fn from_js_string(v: Repr) -> Repr {
        v
    }
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
mod imp {
    pub(super) type Repr = std::rc::Rc<str>;
    pub(super) use Result::Err as ResultJsString;

    pub(super) use super::imp_not_wasm::*;
    pub(super) use std::convert::identity as into_static_owned_string;

    #[cfg(feature = "web")]
    pub(super) fn from_js_string(v: web_sys::js_sys::JsString) -> Repr {
        Repr::from(String::from(v))
    }
}

/// If compiling on `wasm` and crate feature `web` is enabled,
/// this will be a [`JsString`](web_sys::js_sys::JsString).
/// Else, this is a [Rc<str>](std::rc::Rc).
///
/// This type is cheap to [`clone`](Clone::clone).
///
/// The only way to create a [`StringElement`] is to use [`From<&str>`].
///
/// If you have a static owned string like `&'static str`, `String` or `Rc<str>`,
/// just use that as an element.
#[derive(Debug, Clone, PartialEq, Eq)]
// TODO: rename to WebJsString
pub struct StringElement {
    repr: imp::Repr,
    _marker: PhantomData<*mut u8>, // not at all threadsafe
}

#[cfg(feature = "web")]
const _: () = {
    use std::borrow::Cow;

    use web_sys::js_sys::JsString;

    impl StringElement {
        /// This method requires crate feature `web`.
        ///
        /// If compiling on `target_arch = "wasm32"`, this method is a zero cost cast.
        pub fn from_js_string(v: JsString) -> Self {
            Self {
                repr: imp::from_js_string(v),
                _marker: PhantomData,
            }
        }

        pub fn as_js_string(&self) -> Result<&JsString, &std::rc::Rc<str>> {
            imp::ResultJsString(&self.repr)
        }

        pub(crate) fn as_js_string_or_into(&self) -> Cow<JsString> {
            self.as_js_string()
                .map_or_else(|s| Cow::Owned(From::from(&**s)), Cow::Borrowed)
        }

        pub fn into_js_string(self) -> Result<JsString, std::rc::Rc<str>> {
            imp::ResultJsString(self.repr)
        }

        pub(crate) fn into_js_string_or_into(self) -> JsString {
            self.into_js_string().unwrap_or_else(|s| From::from(&*s))
        }
    }
};

impl From<&str> for StringElement {
    fn from(value: &str) -> Self {
        Self {
            repr: From::from(value),
            _marker: PhantomData,
        }
    }
}

impl FromStr for StringElement {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl std::fmt::Display for StringElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.repr, f)
    }
}

mod reactive_value {
    use frender_common::{
        reactive_value::{ReactiveValueWithKind, UncachedNonReactiveValueWithKind},
        value_kind::{KindOfOwned, KindOfRef},
    };

    use super::StringElement;

    impl UncachedNonReactiveValueWithKind for StringElement {
        type UncachedNonReactiveValueKind = KindOfOwned<StringElement>;
    }

    impl ReactiveValueWithKind for StringElement {
        type ReactiveValueKind = KindOfRef<StringElement>;
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::marker::PhantomData;

    use async_str_iter::IntoAsyncStrIterator;
    use frender_ssr::SsrElement;

    use super::StringElement;

    pub struct StringElementIntoStaticOwnedStr(
        super::imp::StaticOwnedStr,
        // marks as not threadsafe
        PhantomData<(
            super::imp_wasm::StaticOwnedStr,
            super::imp_not_wasm::StaticOwnedStr,
        )>,
    );

    impl AsRef<str> for StringElementIntoStaticOwnedStr {
        fn as_ref(&self) -> &str {
            self.0.as_ref()
        }
    }

    impl SsrElement for StringElement {
        type HtmlChildren = frender_ssr::html::encode::Encode<
            frender_ssr::html::escape_safe::Safe,
            async_str_iter::any_str::IterAnyStr<StringElementIntoStaticOwnedStr>,
        >;

        fn into_html_children(self) -> Self::HtmlChildren {
            let s = StringElementIntoStaticOwnedStr(
                super::imp::into_static_owned_string(self.repr),
                PhantomData,
            );
            Self::HtmlChildren::new(
                frender_ssr::html::escape_safe::Safe,
                async_str_iter::any_str::AnyStr(s).into_async_str_iterator(),
            )
        }
    }
}
