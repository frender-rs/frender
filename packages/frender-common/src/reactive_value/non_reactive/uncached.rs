use crate::{reactive_value::ProvideValueOfKind, value_kind::ValueKind};

use super::{
    CachedNonReactiveValue, CachedNonReactiveValueRenderInit, Uncached, UncachedNonReactiveValue,
};

pub struct RenderInit<T>(pub T);

impl<T: ProvideValueOfKind<VK>, VK: ?Sized + ValueKind>
    CachedNonReactiveValueRenderInit<VK, NoCache> for RenderInit<T>
{
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<VK as ValueKind>::Value<'_>) -> Out,
        self::NoCache: &mut NoCache,
    ) -> Out {
        self.0.provide_value_of_kind(renderer)
    }
}

pub struct NoCache;

impl<T: UncachedNonReactiveValue<VK>, VK: ?Sized + ValueKind> CachedNonReactiveValue<VK>
    for Uncached<T>
{
    type Cache = NoCache;
    type RenderInit = RenderInit<T::UncachedIntoProvideValue>;

    type CachedIntoProvideValue = T::UncachedIntoProvideValue;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        self.0.uncached_into_provide_value()
    }

    #[inline(always)]
    fn match_cache(&self, self::NoCache: &Self::Cache) -> bool {
        false
    }

    #[inline(always)]
    fn not_match_cache(&self, self::NoCache: &Self::Cache) -> bool {
        true
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (NoCache, RenderInit(self.cached_into_provide_value()))
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<VK as ValueKind>::Value<'_>) -> Out,
        self::NoCache: &mut Self::Cache,
    ) -> Out {
        self.cached_into_provide_value()
            .provide_value_of_kind(renderer)
    }
}
