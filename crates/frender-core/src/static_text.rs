use std::{borrow::Cow, ops::Deref};

pub trait StaticStr: Deref<Target = str> + Into<Cow<'static, str>> {}

impl StaticStr for &'static str {}
impl StaticStr for String {}
impl StaticStr for Cow<'static, str> {}

/// Marks a *string* doesn't borrow data.
#[repr(transparent)]
pub struct StaticText<S: StaticStr>(pub S);

impl<S: StaticStr> Deref for StaticText<S> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait IntoStaticStr {
    type IntoStaticStr: StaticStr;
    fn into_static_str(self) -> Self::IntoStaticStr;
}

impl IntoStaticStr for &str {
    type IntoStaticStr = String;

    fn into_static_str(self) -> Self::IntoStaticStr {
        self.to_owned()
    }
}

impl IntoStaticStr for String {
    type IntoStaticStr = String;

    fn into_static_str(self) -> Self::IntoStaticStr {
        self
    }
}

impl IntoStaticStr for Cow<'_, str> {
    type IntoStaticStr = String;

    fn into_static_str(self) -> Self::IntoStaticStr {
        match self {
            Cow::Borrowed(s) => s.to_owned(),
            Cow::Owned(s) => s,
        }
    }
}

impl<S: StaticStr> IntoStaticStr for StaticText<S> {
    type IntoStaticStr = S;

    fn into_static_str(self) -> Self::IntoStaticStr {
        self.0
    }
}
