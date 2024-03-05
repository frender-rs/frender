use std::borrow::Borrow;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TempStr<S>(pub S);

impl<S> std::ops::Deref for TempStr<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S: Borrow<str>> Borrow<str> for TempStr<S> {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

pub trait IntoStaticStr {
    type IntoStaticStr: 'static + Borrow<str>;
    fn into_static_str(self) -> Self::IntoStaticStr;

    fn update_into_static_str(self, target: &mut Self::IntoStaticStr)
    where
        Self: Sized,
    {
        *target = self.into_static_str()
    }
}

impl IntoStaticStr for &str {
    type IntoStaticStr = String;
    fn into_static_str(self) -> Self::IntoStaticStr {
        self.to_owned()
    }

    fn update_into_static_str(self, target: &mut Self::IntoStaticStr)
    where
        Self: Sized,
    {
        self.clone_into(target)
    }
}

impl IntoStaticStr for std::borrow::Cow<'_, str> {
    type IntoStaticStr = String;
    fn into_static_str(self) -> Self::IntoStaticStr {
        self.into_owned()
    }

    fn update_into_static_str(self, target: &mut Self::IntoStaticStr)
    where
        Self: Sized,
    {
        match self {
            std::borrow::Cow::Borrowed(v) => v.clone_into(target),
            std::borrow::Cow::Owned(v) => *target = v,
        }
    }
}
