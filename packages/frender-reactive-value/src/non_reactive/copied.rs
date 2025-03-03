//! `T`: CachedNonReactiveValue\<
//! [`KindOfOwned<T>`](crate::value_kind::KindOfOwned),
//! Cache = T\>
//! where `T: 'static + Copy + PartialEq`.
//! `type Value<'_> = T`.

use crate::value_kind::KindOfOwned;

use super::owned::Provide;

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit};

impl<T: 'static + PartialEq + Copy> CachedNonReactiveValue<KindOfOwned<T>> for T {
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = T;
    type RenderInit = RenderInit<T>;

    type CachedIntoProvideValue = Provide<T>;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        Provide(self)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        T::eq(self, cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        T::ne(self, cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self, RenderInit(self))
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(T) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self;
        renderer(self)
    }
}

pub struct RenderInit<T>(pub T);

impl<T: 'static> CachedNonReactiveValueRenderInit<KindOfOwned<T>, T> for RenderInit<T> {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(T) -> Out,
        _: &mut T,
    ) -> Out {
        renderer(self.0)
    }
}
