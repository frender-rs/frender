use std::borrow::Borrow;

use crate::{
    non_reactive::{Uncached, UncachedNonReactiveValue},
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{IntoStatic, TempIntoStatic},
    value_kind::KindOfTempRef,
};

pub trait StaticOrIntoStaticStr {
    type StaticStr: 'static + Borrow<str>;
    fn static_or_into_static_str(self) -> Self::StaticStr;
}

impl<T: 'static + Borrow<str>> StaticOrIntoStaticStr for T {
    type StaticStr = T;

    fn static_or_into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl StaticOrIntoStaticStr for StaticOrTempRef<'_, str> {
    type StaticStr = <Self as IntoStatic<str>>::IntoStatic;

    fn static_or_into_static_str(self) -> Self::StaticStr {
        IntoStatic::<str>::into_static(self)
    }
}
impl<T: IntoStatic<str>> StaticOrIntoStaticStr for TempIntoStatic<T> {
    type StaticStr = T::IntoStatic;

    fn static_or_into_static_str(self) -> Self::StaticStr {
        self.0.into_static()
    }
}
impl<T: UncachedNonReactiveValue<KindOfTempRef<str>> + StaticOrIntoStaticStr> StaticOrIntoStaticStr
    for Uncached<T>
{
    type StaticStr = T::StaticStr;

    fn static_or_into_static_str(self) -> Self::StaticStr {
        self.0.static_or_into_static_str()
    }
}
