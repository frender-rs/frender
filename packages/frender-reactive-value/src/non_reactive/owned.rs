use crate::{
    value_kind::{KindOfOwned, ValueKind},
    ProvideValueOfKind,
};

use super::{CachedNonReactiveValueRenderInit, UncachedNonReactiveValue};

pub type Kind<T> = KindOfOwned<T>;

pub struct Provide<T: 'static>(pub T);

impl<T: 'static> ProvideValueOfKind<Kind<T>> for Provide<T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(T) -> Out) -> Out {
        f(self.0)
    }
}

impl<T: 'static> UncachedNonReactiveValue<Kind<T>> for T {
    type UncachedIntoProvideValue = Provide<T>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        Provide(self)
    }
}

pub struct RenderInit<T>(pub T);

impl<T: 'static> CachedNonReactiveValueRenderInit<Kind<T>, T> for RenderInit<T> {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<Kind<T> as ValueKind>::Value<'_>) -> Out,
        _: &mut T,
    ) -> Out {
        renderer(self.0)
    }
}
