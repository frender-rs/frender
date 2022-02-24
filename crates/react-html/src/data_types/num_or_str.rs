use convert_js::ToJs;
use std::borrow::Cow;

#[derive(ToJs)]
#[convert_js(union)]
pub enum NumOrStr<'a> {
    Num(f64),
    Str(Cow<'a, str>),
}

impl From<f64> for NumOrStr<'static> {
    #[inline]
    fn from(v: f64) -> Self {
        Self::Num(v)
    }
}

impl<'a> From<&'a str> for NumOrStr<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        Self::Str(Cow::Borrowed(v))
    }
}

impl From<String> for NumOrStr<'static> {
    #[inline]
    fn from(v: String) -> Self {
        Self::Str(Cow::Owned(v))
    }
}

impl<'a> From<Cow<'a, str>> for NumOrStr<'a> {
    #[inline]
    fn from(v: Cow<'a, str>) -> Self {
        NumOrStr::Str(v)
    }
}
