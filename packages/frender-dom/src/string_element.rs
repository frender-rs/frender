use std::{marker::PhantomData, str::FromStr};

#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod imp {
    pub(super) type Repr = web_sys::js_sys::JsString;
    pub(super) use Result::Ok as ResultJsString;
}

#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
mod imp {
    pub(super) type Repr = std::rc::Rc<str>;
    pub(super) use Result::Err as ResultJsString;
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
pub struct StringElement {
    repr: imp::Repr,
    _marker: PhantomData<*mut u8>, // not at all threadsafe
}

#[cfg(feature = "web")]
impl StringElement {
    pub fn as_js_string(&self) -> Result<&web_sys::js_sys::JsString, &std::rc::Rc<str>> {
        imp::ResultJsString(&self.repr)
    }

    pub fn into_js_string(self) -> Result<web_sys::js_sys::JsString, std::rc::Rc<str>> {
        imp::ResultJsString(self.repr)
    }
}

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
