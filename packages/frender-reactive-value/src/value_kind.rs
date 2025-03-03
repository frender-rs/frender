use std::marker::PhantomData;

use crate::{static_or_temp_ref::StaticOrTempRef, temp_ref::TempRef};

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
/// - `str` as kind, [`TempStr<&str>`] as value.
///
///   `str: for<'a> ReactiveValueKind<Value<'a> = TempStr<&'a str>>`
///
pub trait ValueKind: 'static + Sized {
    type Value<'a>;
}

enum Never {}
pub struct KindOfOwned<T: 'static>(Never, PhantomData<T>);

// Owned
impl<T: 'static> ValueKind for KindOfOwned<T> {
    type Value<'a> = T;
}

pub struct KindOfTempRef<T: ?Sized + 'static>(Never, PhantomData<&'static T>);

// Ref
impl<T: ?Sized + 'static> ValueKind for KindOfTempRef<T> {
    type Value<'a> = TempRef<'a, T>;
}

pub struct KindOfStaticOrTempRef<T: ?Sized + 'static>(Never, PhantomData<T>);

impl<T: ?Sized + 'static> ValueKind for KindOfStaticOrTempRef<T> {
    type Value<'a> = StaticOrTempRef<'a, T>;
}
