use std::{borrow::Cow, ops::Deref};

pub trait IntoStaticStr {
    type StaticStr: 'static + Deref<Target = str> + Into<Cow<'static, str>>;

    fn into_static_str(self) -> Self::StaticStr;
}

impl IntoStaticStr for &str {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self.to_owned()
    }
}

impl IntoStaticStr for String {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl IntoStaticStr for Cow<'_, str> {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self.into_owned()
    }
}
