use std::{
    borrow::{Borrow as _, Cow},
    ops::Deref,
};

use crate::temp_ref::TempRef;

/// `StaticOrTempRef<T>` implies [`TempIntoStatic<StaticOrTempRef<T>>`](crate::temp_into_static::TempIntoStatic),
/// unlike [`TempRef<T>`](crate::temp_ref::TempRef).
///
/// Note that `StaticOrTempRef<T>` implements `UncachedNonReactiveValue<KindOfStaticOrTempRef<T>>`,
/// but not `UncachedNonReactiveValue<KindOfTempRef<T>>`.
/// For the latter, use [`StaticOrTempRef::to_temp_ref()`] to get a [`TempRef<T>`].
#[derive(Debug)]
pub enum StaticOrTempRef<'a, T: ?Sized + 'static> {
    Static(&'static T),
    Temp(&'a T),
}

impl<'a, T: ?Sized + 'static> AsRef<T> for StaticOrTempRef<'a, T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<'a, T: ?Sized + 'static> Copy for StaticOrTempRef<'a, T> {}
impl<'a, T: ?Sized + 'static> Clone for StaticOrTempRef<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized + 'static> Deref for StaticOrTempRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            StaticOrTempRef::Static(this) => this,
            StaticOrTempRef::Temp(this) => this,
        }
    }
}

impl<'a, T: ?Sized + 'static> StaticOrTempRef<'a, T> {
    pub fn to_ref(self) -> &'a T {
        match self {
            StaticOrTempRef::Static(this) => this,
            StaticOrTempRef::Temp(this) => this,
        }
    }
    pub fn to_temp_ref(self) -> TempRef<'a, T> {
        TempRef(self.to_ref())
    }
}
impl<'a, T: ?Sized + 'static + ToOwned> StaticOrTempRef<'a, T> {
    pub fn to_owned_cow_static(self) -> Cow<'static, T> {
        match self {
            StaticOrTempRef::Static(this) => Cow::Borrowed(this),
            StaticOrTempRef::Temp(this) => Cow::Owned(T::to_owned(this)),
        }
    }

    pub fn clone_into_cow_static(self, target: &mut Cow<'static, T>) {
        match self {
            StaticOrTempRef::Static(this) => *target = Cow::Borrowed(this),
            StaticOrTempRef::Temp(this) => match target {
                Cow::Borrowed(_) => *target = Cow::Owned(this.to_owned()),
                Cow::Owned(target) => this.clone_into(target),
            },
        }
    }
}

impl<T: ?Sized + 'static + ToOwned + PartialEq> PartialEq<StaticOrTempRef<'_, T>>
    for StaticOrTempRef<'_, T>
{
    fn eq(&self, other: &StaticOrTempRef<'_, T>) -> bool {
        T::eq(self, other)
    }

    fn ne(&self, other: &StaticOrTempRef<'_, T>) -> bool {
        T::ne(self, other)
    }
}

impl<T: ?Sized + 'static + ToOwned + PartialEq> PartialEq<Cow<'_, T>> for StaticOrTempRef<'_, T> {
    fn eq(&self, other: &Cow<'_, T>) -> bool {
        T::eq(self, other)
    }

    fn ne(&self, other: &Cow<'_, T>) -> bool {
        T::ne(self, other)
    }
}

impl<'a, T: ?Sized + 'static + ToOwned> From<&'a Cow<'static, T>> for StaticOrTempRef<'a, T> {
    fn from(value: &'a Cow<'static, T>) -> Self {
        match value {
            Cow::Borrowed(value) => StaticOrTempRef::Static(value),
            Cow::Owned(value) => StaticOrTempRef::Temp(value.borrow()),
        }
    }
}
