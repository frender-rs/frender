use crate::{temp_str::TempStr, value_kind::KindOfRef, ProvideValueOfKind};

use super::{
    cache_provide_value::CacheAsRef, CachedNonReactiveValue, CachedNonReactiveValueRenderInit,
};

pub struct RenderInit;

impl<Cache: AsRef<str>> CachedNonReactiveValueRenderInit<KindOfRef<str>, CacheAsRef<Cache>>
    for RenderInit
{
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(&str) -> Out,
        cache: &mut CacheAsRef<Cache>,
    ) -> Out {
        renderer(cache.0.as_ref())
    }
}

impl<S: 'static + AsRef<str> + PartialEq> CachedNonReactiveValue<KindOfRef<str>> for S {
    type CacheCanProvideValue = super::cache_provide_value::CacheCanProvideValue;
    type Cache = CacheAsRef<S>;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideAsRef<S>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideAsRef(self)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        S::eq(self, &cache.0)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        S::ne(self, &cache.0)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (CacheAsRef(self), RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(&str) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        cache.0 = self;
        renderer(cache.0.as_ref())
    }
}

pub struct ProvideAsRef<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> ProvideValueOfKind<KindOfRef<str>> for ProvideAsRef<S> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(&str) -> Out) -> Out {
        f(self.0.as_ref())
    }
}
