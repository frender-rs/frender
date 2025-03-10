use std::{borrow::Borrow, marker::PhantomData};

use frender_reactive_value::{
    non_reactive::{
        CacheCanProvideValue, CacheProvideValue, CachedNonReactiveValue,
        CachedNonReactiveValueRenderInit,
    },
    temp_ref::TempRef,
    value_kind::{KindOfTempRef, ValueKind},
    ProvideValueOfKind,
};

use super::{ConstValue, HasConstValue};

pub struct ConstValueCache<T: ?Sized + HasConstValue>(PhantomData<T>);

impl<T: ?Sized + HasConstValue> CacheProvideValue<KindOfTempRef<str>> for ConstValueCache<T>
where
    T::Value: Borrow<str>,
{
    fn cache_provide_value_with_marker<Out>(
        &self,
        receive: impl FnOnce(<KindOfTempRef<str> as ValueKind>::Value<'_>) -> Out,
        _: CacheCanProvideValue,
    ) -> Out {
        receive(TempRef(T::VALUE.borrow()))
    }
}

pub struct RenderInit;

impl<T: ?Sized + HasConstValue>
    CachedNonReactiveValueRenderInit<KindOfTempRef<str>, ConstValueCache<T>> for RenderInit
where
    T::Value: Borrow<str>,
{
    fn cached_non_reactive_value_render_init<Out>(
        self,
        renderer: impl FnOnce(<KindOfTempRef<str> as ValueKind>::Value<'_>) -> Out,
        _: &mut ConstValueCache<T>,
    ) -> Out {
        renderer(TempRef(T::VALUE.borrow()))
    }
}

pub struct ProvideConstValue<T: ?Sized + HasConstValue>(PhantomData<T>);

impl<T: ?Sized + HasConstValue> ProvideValueOfKind<KindOfTempRef<str>> for ProvideConstValue<T>
where
    T::Value: Borrow<str>,
{
    fn provide_value_of_kind<Out>(
        self,
        f: impl FnOnce(<KindOfTempRef<str> as ValueKind>::Value<'_>) -> Out,
    ) -> Out {
        f(TempRef(T::VALUE.borrow()))
    }
}

impl<T: HasConstValue> CachedNonReactiveValue<KindOfTempRef<str>> for ConstValue<T>
where
    T::Value: Borrow<str>,
{
    type CacheCanProvideValue = CacheCanProvideValue;

    type Cache = ConstValueCache<T>;

    type RenderInit = RenderInit;

    type CachedIntoProvideValue = ProvideConstValue<T>;

    fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
        ProvideConstValue(PhantomData)
    }

    fn match_cache(&self, _: &Self::Cache) -> bool {
        true
    }

    fn not_match_cache(&self, _: &Self::Cache) -> bool {
        false
    }

    fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
        (ConstValueCache(PhantomData), RenderInit)
    }

    fn update_into_cache_and_render<Out>(
        self,
        renderer: impl FnOnce(<KindOfTempRef<str> as ValueKind>::Value<'_>) -> Out,
        _: &mut Self::Cache,
    ) -> Out {
        renderer(TempRef(T::VALUE.borrow()))
    }
}
