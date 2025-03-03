use std::borrow::Borrow;

use crate::{temp_ref::TempRef, value_kind::KindOfTempRef};

use super::{
    temp_into_static::{ProvideBorrow, RenderInit},
    CachedNonReactiveValue,
};

impl<S: 'static + Borrow<str> + PartialEq> CachedNonReactiveValue<KindOfTempRef<str>> for S {
    type CacheCanProvideValue = super::cache_provide_value::CacheCanProvideValue;
    type Cache = S;
    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideBorrow<S>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideBorrow(self)
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
        renderer: impl FnOnce(TempRef<str>) -> Out,
        cache: &mut Self::Cache,
    ) -> Out {
        *cache = self;
        renderer(TempRef(S::borrow(cache)))
    }
}
