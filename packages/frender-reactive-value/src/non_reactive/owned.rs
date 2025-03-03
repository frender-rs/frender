use crate::{value_kind::KindOfOwned, ProvideValueOfKind};

use super::UncachedNonReactiveValue;

pub struct Provide<T: 'static>(pub T);

impl<T: 'static> ProvideValueOfKind<KindOfOwned<T>> for Provide<T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(T) -> Out) -> Out {
        f(self.0)
    }
}

impl<T: 'static> UncachedNonReactiveValue<KindOfOwned<T>> for T {
    type UncachedIntoProvideValue = Provide<T>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        Provide(self)
    }
}
