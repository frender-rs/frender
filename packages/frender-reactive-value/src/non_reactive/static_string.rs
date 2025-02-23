use crate::{temp_str::TempStr, ProvideValueOfKind};

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit};

pub struct RenderInit;

impl<Cache: AsRef<str>> CachedNonReactiveValueRenderInit<str, Cache> for RenderInit {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Cache,
    ) -> Out {
        renderer(TempStr(cache.as_ref()))
    }
}

impl<S: 'static + AsRef<str> + PartialEq> CachedNonReactiveValue<str> for S {
    type CacheCanProvideValue = super::cache_provide_value::CacheCanProvideValue;
    type Cache = S;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideAsRef<S>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideAsRef(self)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        S::eq(self, cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        S::ne(self, cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self, RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self;
        renderer(TempStr(cache.as_ref()))
    }
}

pub struct ProvideAsRef<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> ProvideValueOfKind<str> for ProvideAsRef<S> {
    fn provide_value_of_kind<Out>(
        self,
        f: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
    ) -> Out {
        f(TempStr(self.0.as_ref()))
    }
}
