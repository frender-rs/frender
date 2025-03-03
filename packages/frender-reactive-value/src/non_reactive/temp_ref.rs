use crate::{temp_ref::TempRef, value_kind::KindOfTempRef, ProvideValueOfKind};

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit, UncachedNonReactiveValue};

pub struct ProvideTempRef<'a, T: ?Sized>(pub &'a T);

impl<T: ?Sized> ProvideValueOfKind<KindOfTempRef<T>> for ProvideTempRef<'_, T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempRef<T>) -> Out) -> Out {
        f(TempRef(self.0))
    }
}

impl<'a, T: ?Sized> UncachedNonReactiveValue<KindOfTempRef<T>> for TempRef<'a, T> {
    type UncachedIntoProvideValue = ProvideTempRef<'a, T>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        ProvideTempRef(self.0)
    }
}

pub struct RenderInit<'a, T: ?Sized + 'static>(&'a T);

impl<'a, T: ?Sized + 'static> CachedNonReactiveValueRenderInit<KindOfTempRef<T>, &'a T>
    for RenderInit<'a, T>
{
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<KindOfTempRef<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        _: &mut &'a T,
    ) -> Out {
        renderer(TempRef(self.0))
    }
}

impl<'a, T: ?Sized + 'static + PartialEq> CachedNonReactiveValue<KindOfTempRef<T>>
    for TempRef<'a, T>
{
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = &'a T;
    type RenderInit = RenderInit<'a, T>;
    type CachedIntoProvideValue = ProvideTempRef<'a, T>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideTempRef(self.0)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        T::eq(self.0, cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        T::ne(self.0, cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self.0, RenderInit(self.0))
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<KindOfTempRef<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self.0;
        renderer(TempRef(self.0))
    }
}
