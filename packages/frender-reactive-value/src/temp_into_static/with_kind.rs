use std::borrow::Cow;

use frender_common::impl_many;

use crate::static_or_temp_ref::StaticOrTempRef;

use super::{cheap_clone_pointer::KnownCheapClonePointer, IntoStatic, ToStatic};

pub trait IntoStaticWithKind: IntoStatic<Self::IntoStaticValue> {
    type IntoStaticValue: ?Sized + 'static;
}

pub trait ToStaticWithKind: ToStatic<Self::ToStaticValue> {
    type ToStaticValue: ?Sized + 'static;
}

impl<T: ?Sized + ToStaticWithKind> IntoStaticWithKind for &T {
    type IntoStaticValue = T::ToStaticValue;
}

impl_many!(
    impl<__> ToStaticWithKind
        for each_of![
            //
            str,
        ]
    {
        type ToStaticValue = Self;
    }
);

impl<T: ?Sized + 'static + ToOwned> ToStaticWithKind for Cow<'static, T> {
    // Note
    type ToStaticValue = Self;
}

impl<T: KnownCheapClonePointer<Target = str>> ToStaticWithKind for T {
    // Note
    type ToStaticValue = Self;
}

impl<V: ?Sized + 'static + ToOwned> IntoStaticWithKind for Cow<'_, V> {
    type IntoStaticValue = V;
}

impl<V: ?Sized + 'static + ToOwned> IntoStaticWithKind for StaticOrTempRef<'_, V> {
    type IntoStaticValue = Cow<'static, V>;
}
