use crate::temp_str::{IntoStaticStrCache, TempStr};

use super::{
    static_string::{ProvideAsRef, RenderInit},
    CachedNonReactiveValue, UncachedNonReactiveValue,
};

impl<S: IntoStaticStrCache> CachedNonReactiveValue<str> for TempStr<S> {
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = S::StaticStrCache;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideAsRef<S::StaticStrCache>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideAsRef(self.0.into_static_str_cache())
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        S::StaticStrCache::eq(cache, &self.0)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        S::StaticStrCache::ne(cache, &self.0)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self.0.into_static_str_cache(), RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        self.0.update_into_static_str_cache(cache);
        renderer(TempStr(cache.as_ref()))
    }
}

impl<'a> UncachedNonReactiveValue<str> for TempStr<&'a str> {
    type UncachedIntoProvideValue = ProvideAsRef<&'a str>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        ProvideAsRef(self.0)
    }
}

impl UncachedNonReactiveValue<str> for &str {
    type UncachedIntoProvideValue = ProvideAsRef<Self>;

    fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
        ProvideAsRef(self)
    }
}
