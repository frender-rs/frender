use std::{
    borrow::{Borrow as _, Cow},
    marker::PhantomData,
    ops::Deref,
};

use crate::TempStr;

/// Notable kinds and values:
///
/// - Copied
///
///     `T` as both Kind and Value where `T: 'static + Copy`.
///
///     `T: 'static + Copy` implies `T: ReactiveValueKind<Value<'_> = T>`
///
/// - Cloned: [`KindOfOwned<T>`] as Kind, `T` as Value.
///
/// - [`KindOfStaticRefOrTempOwned<T>`] as Kind, [`StaticRefOrTempOwned<'_, T>`] as Value.
///
/// - Ref: [`KindOfRef<T>`] as Kind, `&T` as Value.
///
/// - `str` as kind, [`TempStr<&str>`](crate::TempStr) as value.
///
///   `str: for<'a> ReactiveValueKind<Value<'a> = TempStr<&'a str>>`
///
pub trait ValueKind: 'static {
    type Value<'a>;
}

enum Never {}
pub struct KindOfOwned<T: 'static>(Never, PhantomData<T>);

// Owned
impl<T: 'static> ValueKind for KindOfOwned<T> {
    type Value<'a> = T;
}

pub struct KindOfRef<T: ?Sized + 'static>(Never, PhantomData<&'static T>);

// Ref
impl<T: ?Sized + 'static> ValueKind for KindOfRef<T> {
    type Value<'a> = &'a T;
}

impl ValueKind for str {
    type Value<'a> = TempStr<&'a str>;
}

#[derive(Debug)]
pub enum StaticRefOrTempOwned<'a, T: ?Sized + 'static + ToOwned> {
    Static(&'static T),
    Temp(&'a T::Owned),
}

impl<'a, T: ?Sized + 'static + ToOwned> AsRef<T> for StaticRefOrTempOwned<'a, T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<'a, T: ?Sized + 'static + ToOwned> Copy for StaticRefOrTempOwned<'a, T> {}

impl<'a, T: ?Sized + 'static + ToOwned> Clone for StaticRefOrTempOwned<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: ?Sized + 'static + ToOwned> Deref for StaticRefOrTempOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            StaticRefOrTempOwned::Static(this) => this,
            StaticRefOrTempOwned::Temp(this) => (*this).borrow(),
        }
    }
}

impl<'a, T: ?Sized + 'static + ToOwned> StaticRefOrTempOwned<'a, T> {
    pub fn to_owned_cow_static(self) -> Cow<'static, T> {
        match self {
            StaticRefOrTempOwned::Static(this) => Cow::Borrowed(this),
            StaticRefOrTempOwned::Temp(this) => Cow::Owned(T::to_owned(this.borrow())),
        }
    }

    pub fn clone_into_cow_static(self, target: &mut Cow<'static, T>) {
        match self {
            StaticRefOrTempOwned::Static(this) => *target = Cow::Borrowed(this),
            StaticRefOrTempOwned::Temp(this) => {
                let this = this.borrow();
                match target {
                    Cow::Borrowed(_) => *target = Cow::Owned(this.to_owned()),
                    Cow::Owned(target) => this.clone_into(target),
                }
            }
        }
    }
}

impl<'a, T: ?Sized + 'static + ToOwned + PartialEq> PartialEq<Cow<'_, T>>
    for StaticRefOrTempOwned<'a, T>
{
    fn eq(&self, other: &Cow<'_, T>) -> bool {
        T::eq(self, other)
    }

    fn ne(&self, other: &Cow<'_, T>) -> bool {
        T::ne(self, other)
    }
}

impl<'a, T: ?Sized + 'static + ToOwned> From<&'a Cow<'static, T>> for StaticRefOrTempOwned<'a, T> {
    fn from(value: &'a Cow<'static, T>) -> Self {
        match value {
            Cow::Borrowed(value) => StaticRefOrTempOwned::Static(value),
            Cow::Owned(value) => StaticRefOrTempOwned::Temp(value),
        }
    }
}

pub struct KindOfStaticRefOrTempOwned<T: ?Sized + 'static + ToOwned>(Never, PhantomData<T>);

impl<T: ?Sized + 'static + ToOwned> ValueKind for KindOfStaticRefOrTempOwned<T> {
    type Value<'a> = StaticRefOrTempOwned<'a, T>;
}
