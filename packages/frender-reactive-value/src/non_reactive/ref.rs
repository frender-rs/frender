use crate::{value_kind::KindOfRef, ProvideValueOfKind};

use super::UncachedNonReactiveValue;

pub type Kind<T> = KindOfRef<T>;

pub struct Provide<'a, T: ?Sized>(pub &'a T);

impl<T: ?Sized> ProvideValueOfKind<Kind<T>> for Provide<'_, T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(&T) -> Out) -> Out {
        f(self.0)
    }
}

impl<'a, T: ?Sized> UncachedNonReactiveValue<Kind<T>> for &'a T {
    type UncachedIntoProvideValue = Provide<'a, T>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        Provide(self)
    }
}
