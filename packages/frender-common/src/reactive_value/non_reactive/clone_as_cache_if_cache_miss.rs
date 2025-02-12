pub use super::r#ref::{Kind, Provide};

use std::borrow::Borrow as _;

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit, CloneIfCacheMiss};

pub struct RenderInit<'a, T: ?Sized>(&'a T);

impl<T: ?Sized, Cache> CachedNonReactiveValueRenderInit<Kind<T>, Cache> for RenderInit<'_, T> {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(&T) -> Out,
        _: &mut Cache,
    ) -> Out {
        renderer(self.0)
    }
}

impl<'a, T: 'static + ?Sized + ToOwned + PartialEq> CachedNonReactiveValue<Kind<T>>
    for CloneIfCacheMiss<&'a T>
{
    type Cache = T::Owned;
    type RenderInit = RenderInit<'a, T>;

    type CachedIntoProvideValue = Provide<'a, T>;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        Provide(self.0)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        T::eq(&self.0, T::Owned::borrow(cache))
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        T::ne(&self.0, T::Owned::borrow(cache))
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self.0.to_owned(), RenderInit(self.0))
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        self.0.clone_into(cache);
        renderer(self.0)
    }
}
