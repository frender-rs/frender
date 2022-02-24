use convert_js::ToJs;
use std::borrow::Cow;

#[derive(ToJs)]
#[convert_js(union)]
pub enum BoolOrStr<'a> {
    Bool(bool),
    Str(&'a str),
}

impl From<bool> for BoolOrStr<'static> {
    #[inline]
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl<'a> From<&'a str> for BoolOrStr<'a> {
    #[inline]
    fn from(v: &'a str) -> Self {
        Self::Str(v)
    }
}
