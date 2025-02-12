use crate::{
    reactive_value::ProvideValueOfKind, strings::CsrStr, IntoStaticStrCache, TempStr, ToAsRefStr,
};

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit};

pub struct Provide<T: CsrStr>(pub T);

impl<T: CsrStr> ProvideValueOfKind<str> for Provide<T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempStr<&str>) -> Out) -> Out {
        // TODO: Uncached<impl CsrStr> should be used
        f(TempStr(
            self.0
                .into_into_static_str_cache()
                .into_static_str_cache()
                .to_as_ref_str()
                .as_ref(),
        ))
    }
}

pub struct RenderInit;

impl<Cache: ToAsRefStr> CachedNonReactiveValueRenderInit<str, Cache> for RenderInit {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Cache,
    ) -> Out {
        renderer(TempStr(cache.to_as_ref_str().as_ref()))
    }
}

impl<T: CsrStr> CachedNonReactiveValue<str> for T {
    type Cache = T::StaticStrCache;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = Provide<T>;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        Provide(self)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        self.match_static_str_cache(cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        self.not_match_static_str_cache(cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (
            self.into_into_static_str_cache().into_static_str_cache(),
            RenderInit,
        )
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<str as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        self.into_into_static_str_cache()
            .update_into_static_str_cache(cache);

        renderer(TempStr(cache.to_as_ref_str().as_ref()))
    }
}
