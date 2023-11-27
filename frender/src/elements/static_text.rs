use std::{borrow::Cow, ops::Deref};

pub trait StaticStr: 'static + Deref<Target = str> + Into<Cow<'static, str>> {}

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
