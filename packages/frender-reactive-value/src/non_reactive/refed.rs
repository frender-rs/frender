//! `T` as ReactiveValue, [`KindOfRef<T>`] as Kind,
//! `T` as Cache, `&T` as Value,
//! where `T: 'static + PartialEq`

use crate::value_kind::KindOfRef;

pub type Kind<T> = KindOfRef<T>;

use crate::ProvideValueOfKind;

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit};

pub struct Provide<T>(pub T);

impl<T> ProvideValueOfKind<Kind<T>> for Provide<T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(&T) -> Out) -> Out {
        f(&self.0)
    }
}

pub struct RenderInit;

impl<T> CachedNonReactiveValueRenderInit<Kind<T>, T> for RenderInit {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(&T) -> Out,
        cache: &mut T,
    ) -> Out {
        renderer(cache)
    }
}

impl<T: 'static + PartialEq> CachedNonReactiveValue<Kind<T>> for T {
    type CacheCanProvideValue = super::CacheCanProvideValue;
    type Cache = T;
    type RenderInit = RenderInit;

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
        (self, RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self;
        renderer(cache)
    }
}
