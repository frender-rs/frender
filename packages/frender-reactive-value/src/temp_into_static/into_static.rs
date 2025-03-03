use std::borrow::{Borrow, Cow};

use crate::static_or_temp_ref::StaticOrTempRef;

use super::cheap_clone_pointer::KnownCheapClonePointer;

pub trait IntoStatic<V: ?Sized + 'static> {
    type IntoStatic: 'static + Borrow<V>;
    fn into_static(self) -> Self::IntoStatic
    where
        Self: Sized;
}

pub trait ToStatic<V: ?Sized + 'static>: Borrow<V> {
    type ToStatic: 'static + Borrow<V>;
    fn to_static(&self) -> Self::ToStatic;
}

impl<T: ?Sized + ToStatic<V>, V: ?Sized + 'static> IntoStatic<V> for &T {
    type IntoStatic = T::ToStatic;

    fn into_static(self) -> Self::IntoStatic {
        T::to_static(self)
    }
}

/// This implementation includes `str` and `Cow<'static, str>`
impl<V: ?Sized + 'static + ToOwned> ToStatic<V> for V {
    type ToStatic = V::Owned;

    fn to_static(&self) -> Self::ToStatic {
        V::to_owned(self)
    }
}

impl<T: ?Sized + 'static + ToOwned> ToStatic<T> for Cow<'static, T> {
    type ToStatic = Self;

    fn to_static(&self) -> Self::ToStatic {
        Self::clone(self)
    }
}

impl<T: KnownCheapClonePointer<Target = str>> ToStatic<str> for T {
    type ToStatic = T;

    fn to_static(&self) -> Self::ToStatic {
        Self::clone(self)
    }
}

impl<V: ?Sized + 'static + ToOwned> IntoStatic<V> for Cow<'_, V> {
    type IntoStatic = V::Owned;

    fn into_static(self) -> Self::IntoStatic {
        self.into_owned()
    }
}

impl<T: ?Sized + 'static + ToOwned> IntoStatic<T> for StaticOrTempRef<'_, T> {
    type IntoStatic = Cow<'static, T>;

    fn into_static(self) -> Self::IntoStatic
    where
        Self: Sized,
    {
        self.to_owned_cow_static()
    }
}

impl<T: ?Sized + 'static + ToOwned> IntoStatic<Cow<'static, T>> for StaticOrTempRef<'_, T> {
    type IntoStatic = Cow<'static, T>;

    fn into_static(self) -> Self::IntoStatic
    where
        Self: Sized,
    {
        self.to_owned_cow_static()
    }
}
