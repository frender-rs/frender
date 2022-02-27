use convert_js::ToJs;

#[derive(ToJs)]
#[convert_js(union)]
pub enum NumOrStr<'a> {
    Num(f64),
    Str(&'a str),
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
        Self::Str(v)
    }
}
