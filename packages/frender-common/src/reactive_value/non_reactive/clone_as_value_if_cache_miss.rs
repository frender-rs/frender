//! [`CloneIfCacheMiss<T>`]\: CachedNonReactiveValue\<
//! [`KindOfOwned<T>`](crate::value_kind::KindOfOwned),
//! Cache = T\>
//! where `T: 'static + Clone + PartialEq`.
//! `type Value<'_> = T`.

pub use super::owned::{Kind, Provide, RenderInit};

use crate::value_kind::ValueKind;

use super::{CachedNonReactiveValue, CloneIfCacheMiss};

impl<T: 'static + Clone + PartialEq> CachedNonReactiveValue<Kind<T>> for CloneIfCacheMiss<T> {
    type Cache = T;
    type RenderInit = RenderInit<T>;

    type CachedIntoProvideValue = Provide<T>;
    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        Provide(self.0)
    }

    fn match_cache(&self, cache: &Self::Cache) -> bool {
        T::eq(&self.0, cache)
    }

    fn not_match_cache(&self, cache: &Self::Cache) -> bool {
        T::ne(&self.0, cache)
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (self.0.clone(), RenderInit(self.0))
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<Kind<T> as ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        cache.clone_from(&self.0);
        renderer(self.0)
    }
}
