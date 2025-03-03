//! `T` as ReactiveValue, [`KindOfRef<T>`] as Kind,
//! `T` as Cache, `&T` as Value,
//! where `T: 'static + PartialEq`

use crate::{temp_ref::TempRef, value_kind::KindOfTempRef};

use crate::ProvideValueOfKind;

use super::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit};

pub struct Provide<T>(pub T);

impl<T> ProvideValueOfKind<KindOfTempRef<T>> for Provide<T> {
    fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempRef<T>) -> Out) -> Out {
        f(TempRef(&self.0))
    }
}

pub struct RenderInit;

impl<T> CachedNonReactiveValueRenderInit<KindOfTempRef<T>, T> for RenderInit {
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(TempRef<T>) -> Out,
        cache: &mut T,
    ) -> Out {
        renderer(TempRef(cache))
    }
}

impl<T: 'static + PartialEq> CachedNonReactiveValue<KindOfTempRef<T>> for T {
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
        renderer: impl FnOnce(<KindOfTempRef<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self;
        renderer(TempRef(cache))
    }
}
