use crate::{reactive_value::ProvideValueOfKind, TempStr, ToAsRefStr};

use super::UncachedNonReactiveValue;

pub struct Provide<S: ToAsRefStr>(pub S);

impl<S: ToAsRefStr> ProvideValueOfKind<str> for Provide<S> {
    fn provide_value_of_kind<Out>(
        self,
        f: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
    ) -> Out {
        f(TempStr(self.0.to_as_ref_str().as_ref()))
    }
}

impl<S: ToAsRefStr> UncachedNonReactiveValue<str> for TempStr<S> {
    type UncachedIntoProvideValue = Provide<S>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        Provide(self.0)
    }
}

impl UncachedNonReactiveValue<str> for &str {
    type UncachedIntoProvideValue = Provide<Self>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        Provide(self)
    }
}
