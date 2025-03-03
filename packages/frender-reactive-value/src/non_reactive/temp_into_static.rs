use std::borrow::Borrow;

use crate::{
    temp_into_static::{IntoStaticCache, TempIntoStatic, UncachedTempIntoStatic},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
    ProvideValueOfKind,
};

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit, UncachedNonReactiveValue};

impl<T: UncachedTempIntoStatic<V>, V: ?Sized + 'static> UncachedNonReactiveValue<KindOfTempRef<V>>
    for TempIntoStatic<T>
{
    type UncachedIntoProvideValue = ProvideUncachedTempIntoStatic<T>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        ProvideUncachedTempIntoStatic(self.0)
    }
}

impl<T: IntoStaticCache<V>, V: ?Sized> CachedNonReactiveValue<KindOfTempRef<V>>
    for TempIntoStatic<T>
{
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = T::IntoStaticCache;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideBorrow<T::IntoStaticCache>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideBorrow(self.0.into_static_str_cache())
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        self.0.match_into_static_cache(cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        self.0.not_match_into_static_cache(cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self.0.into_static_str_cache(), RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(TempRef<V>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        self.0.update_into_static_str_cache(cache);
        renderer(TempRef(Self::Cache::borrow(cache)))
    }
}

pub struct RenderInit;

impl<Cache: Borrow<V>, V: ?Sized> CachedNonReactiveValueRenderInit<KindOfTempRef<V>, Cache>
    for RenderInit
{
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(TempRef<V>) -> Out,
        cache: &mut Cache,
    ) -> Out {
        renderer(TempRef(Cache::borrow(cache)))
    }
}

pub struct ProvideBorrow<S>(pub S);

impl<S: Borrow<V>, V: ?Sized> ProvideValueOfKind<KindOfTempRef<V>> for ProvideBorrow<S> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempRef<V>) -> Out) -> Out {
        f(TempRef(self.0.borrow()))
    }
}

pub struct ProvideUncachedTempIntoStatic<S>(pub S);

impl<S: UncachedTempIntoStatic<V>, V: ?Sized> ProvideValueOfKind<KindOfTempRef<V>>
    for ProvideUncachedTempIntoStatic<S>
{
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempRef<V>) -> Out) -> Out {
        f(TempRef(self.0.uncached_as_ref_temp()))
    }
}
