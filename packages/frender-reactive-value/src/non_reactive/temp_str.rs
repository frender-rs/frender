use crate::{
    temp_str::{IntoStaticStrCache, TempStr},
    value_kind::KindOfRef,
};

use super::{
    cache_provide_value::CacheAsRef,
    static_string::{ProvideAsRef, RenderInit},
    CachedNonReactiveValue, UncachedNonReactiveValue,
};

impl<S: IntoStaticStrCache> CachedNonReactiveValue<KindOfRef<str>> for TempStr<S> {
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = CacheAsRef<S::StaticStrCache>;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideAsRef<S::StaticStrCache>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideAsRef(self.0.into_static_str_cache())
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        S::StaticStrCache::eq(&cache.0, &self.0)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        S::StaticStrCache::ne(&cache.0, &self.0)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (CacheAsRef(self.0.into_static_str_cache()), RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(&str) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        self.0.update_into_static_str_cache(&mut cache.0);
        renderer(cache.0.as_ref())
    }
}

impl<'a> UncachedNonReactiveValue<KindOfRef<str>> for TempStr<&'a str> {
    type UncachedIntoProvideValue = ProvideAsRef<&'a str>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        ProvideAsRef(self.0)
    }
}
