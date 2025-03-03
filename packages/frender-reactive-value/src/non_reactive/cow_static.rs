//! <ul><li><code>
//! Cow<'static, T>: CachedNonReactiveValue<KindOfStaticRefOrTempOwned<T>, Cache = Cow<'static, T>>
//!
//!
//!
//! KindOfOwned<T>`](crate::value_kind::KindOfOwned),
//! Cache = T\>
//! where `T: 'static + Clone + PartialEq`.
//! `type Value<'_> = T`.
//!
//! aasf
//! </code></li></ul>

mod cached_refed_cow_static {
    use std::borrow::Cow;

    use crate::{
        non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit},
        static_or_temp_ref::StaticOrTempRef,
        value_kind::KindOfStaticOrTempRef,
        ProvideValueOfKind,
    };

    pub struct Provide<T: ?Sized + 'static + ToOwned>(Cow<'static, T>);

    impl<T: ?Sized + 'static + ToOwned> ProvideValueOfKind<KindOfStaticOrTempRef<T>> for Provide<T> {
        fn provide_value_of_kind<Out>(self, f: impl FnOnce(StaticOrTempRef<'_, T>) -> Out) -> Out {
            f(From::from(&self.0))
        }
    }

    pub struct RenderInit;

    impl<T: ?Sized + 'static + ToOwned + PartialEq>
        CachedNonReactiveValueRenderInit<KindOfStaticOrTempRef<T>, Cow<'static, T>> for RenderInit
    {
        fn cached_non_reactive_value_render_init<Out>(
            self,
            renderer: impl FnOnce(
                <KindOfStaticOrTempRef<T> as crate::value_kind::ValueKind>::Value<'_>,
            ) -> Out,
            cache: &mut Cow<'static, T>,
        ) -> Out {
            renderer(From::<&_>::from(cache))
        }
    }

    impl<T: ?Sized + 'static + ToOwned + PartialEq> CachedNonReactiveValue<KindOfStaticOrTempRef<T>>
        for Cow<'static, T>
    {
        type CacheCanProvideValue = super::super::CacheCanProvideValue;
        type Cache = Self;

        type RenderInit = RenderInit;

        type CachedIntoProvideValue = Provide<T>;
        fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
            Provide(self)
        }

        fn match_cache(&self, cache: &Self::Cache) -> bool {
            Self::eq(self, cache)
        }

        fn not_match_cache(&self, cache: &Self::Cache) -> bool {
            Self::ne(self, cache)
        }

        fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
            (self, RenderInit)
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(
                <KindOfStaticOrTempRef<T> as crate::value_kind::ValueKind>::Value<'_>,
            ) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            *cache = self;
            renderer(From::<&_>::from(cache))
        }
    }
}

mod uncached_ref_cow_static {
    use std::borrow::Cow;

    use crate::{
        non_reactive::UncachedNonReactiveValue, static_or_temp_ref::StaticOrTempRef,
        value_kind::KindOfStaticOrTempRef, ProvideValueOfKind,
    };

    pub struct Provide<'a, T: ?Sized + 'static + ToOwned>(pub(super) &'a Cow<'static, T>);

    impl<T: ?Sized + 'static + ToOwned> ProvideValueOfKind<KindOfStaticOrTempRef<T>>
        for Provide<'_, T>
    {
        fn provide_value_of_kind<Out>(self, f: impl FnOnce(StaticOrTempRef<'_, T>) -> Out) -> Out {
            f(From::from(self.0))
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned> UncachedNonReactiveValue<KindOfStaticOrTempRef<T>>
        for &'a Cow<'static, T>
    {
        type UncachedIntoProvideValue = Provide<'a, T>;

        fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
            Provide(self)
        }
    }
}

mod uncached_static_or_temp_ref {
    use crate::{
        non_reactive::UncachedNonReactiveValue,
        static_or_temp_ref::StaticOrTempRef,
        temp_ref::TempRef,
        value_kind::{KindOfStaticOrTempRef, KindOfTempRef},
        ProvideValueOfKind,
    };

    pub struct Provide<'a, T: ?Sized + 'static>(pub(super) StaticOrTempRef<'a, T>);

    impl<T: ?Sized + 'static> ProvideValueOfKind<KindOfStaticOrTempRef<T>> for Provide<'_, T> {
        fn provide_value_of_kind<Out>(self, f: impl FnOnce(StaticOrTempRef<'_, T>) -> Out) -> Out {
            f(self.0)
        }
    }

    impl<T: ?Sized + 'static> ProvideValueOfKind<KindOfTempRef<T>> for Provide<'_, T> {
        fn provide_value_of_kind<Out>(self, f: impl FnOnce(TempRef<'_, T>) -> Out) -> Out {
            f(TempRef(&self.0))
        }
    }

    impl<'a, T: ?Sized + 'static> UncachedNonReactiveValue<KindOfStaticOrTempRef<T>>
        for StaticOrTempRef<'a, T>
    {
        type UncachedIntoProvideValue = Provide<'a, T>;

        fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
            Provide(self)
        }
    }
}

mod partially_cached {
    use std::borrow::Cow;

    use crate::{
        non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit},
        static_or_temp_ref::StaticOrTempRef,
        temp_ref::TempRef,
        value_kind::{KindOfStaticOrTempRef, KindOfTempRef},
    };

    use super::{
        super::cache_provide_value::CacheCanNotProvideValue,
        uncached_ref_cow_static::Provide as ProvideRefCow,
        uncached_static_or_temp_ref::Provide as ProvideRef,
    };

    pub enum PartialCacheCowStatic<T: ?Sized + 'static> {
        StaticBorrowed(&'static T),
        TempOwned,
    }

    impl<T: ?Sized + 'static> Copy for PartialCacheCowStatic<T> {}

    impl<T: ?Sized + 'static> Clone for PartialCacheCowStatic<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T: ?Sized + 'static> PartialCacheCowStatic<T> {
        fn from_ref_cow(this: &Cow<'static, T>) -> Self
        where
            T: ToOwned,
        {
            match this {
                Cow::Borrowed(this) => PartialCacheCowStatic::StaticBorrowed(this),
                Cow::Owned(_) => PartialCacheCowStatic::TempOwned,
            }
        }

        fn from_ref(this: StaticOrTempRef<T>) -> Self
        where
            T: ToOwned,
        {
            match this {
                StaticOrTempRef::Static(this) => Self::StaticBorrowed(this),
                StaticOrTempRef::Temp(_) => Self::TempOwned,
            }
        }

        fn cache_match(self, v: &T) -> bool
        where
            T: PartialEq,
        {
            match self {
                PartialCacheCowStatic::StaticBorrowed(cache) => T::eq(cache, v),
                PartialCacheCowStatic::TempOwned => false,
            }
        }
        fn cache_not_match(self, v: &T) -> bool
        where
            T: PartialEq,
        {
            match self {
                PartialCacheCowStatic::StaticBorrowed(cache) => T::ne(cache, v),
                PartialCacheCowStatic::TempOwned => true,
            }
        }
    }

    pub struct RenderInit<'a, T: ?Sized + 'static + ToOwned>(StaticOrTempRef<'a, T>);

    impl<'a, T: ?Sized + 'static + ToOwned>
        CachedNonReactiveValueRenderInit<KindOfStaticOrTempRef<T>, PartialCacheCowStatic<T>>
        for RenderInit<'a, T>
    {
        fn cached_non_reactive_value_render_init<Out>(
            self,
            renderer: impl FnOnce(
                <KindOfStaticOrTempRef<T> as crate::value_kind::ValueKind>::Value<'_>,
            ) -> Out,
            _: &mut PartialCacheCowStatic<T>,
        ) -> Out {
            renderer(self.0)
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned>
        CachedNonReactiveValueRenderInit<KindOfTempRef<T>, PartialCacheCowStatic<T>>
        for RenderInit<'a, T>
    {
        fn cached_non_reactive_value_render_init<Out>(
            self,
            renderer: impl FnOnce(<KindOfTempRef<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            _: &mut PartialCacheCowStatic<T>,
        ) -> Out {
            renderer(TempRef(&self.0))
        }
    }

    /// Partially cached
    impl<'a, T: ?Sized + 'static + ToOwned + PartialEq>
        CachedNonReactiveValue<KindOfStaticOrTempRef<T>> for &'a Cow<'static, T>
    {
        type CacheCanProvideValue = CacheCanNotProvideValue;
        type Cache = PartialCacheCowStatic<T>;

        type RenderInit = RenderInit<'a, T>;

        type CachedIntoProvideValue = ProvideRefCow<'a, T>;
        fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
            ProvideRefCow(self)
        }

        fn match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_match(self)
        }

        fn not_match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_not_match(self)
        }

        fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
            (
                PartialCacheCowStatic::from_ref_cow(self),
                RenderInit(From::from(self)),
            )
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(StaticOrTempRef<T>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            *cache = PartialCacheCowStatic::from_ref_cow(self);
            renderer(From::from(self))
        }
    }

    /// Partially cached
    impl<'a, T: ?Sized + 'static + ToOwned + PartialEq>
        CachedNonReactiveValue<KindOfStaticOrTempRef<T>> for StaticOrTempRef<'a, T>
    {
        type CacheCanProvideValue = CacheCanNotProvideValue;
        type Cache = PartialCacheCowStatic<T>;

        type RenderInit = RenderInit<'a, T>;

        type CachedIntoProvideValue = ProvideRef<'a, T>;
        fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
            ProvideRef(self)
        }

        fn match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_match(self)
        }

        fn not_match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_not_match(self)
        }

        fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
            (PartialCacheCowStatic::from_ref(self), RenderInit(self))
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(StaticOrTempRef<T>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            *cache = PartialCacheCowStatic::from_ref(self);
            renderer(self)
        }
    }

    /// Partially cached
    impl<'a, T: ?Sized + 'static + ToOwned + PartialEq> CachedNonReactiveValue<KindOfTempRef<T>>
        for StaticOrTempRef<'a, T>
    {
        type CacheCanProvideValue = CacheCanNotProvideValue;
        type Cache = PartialCacheCowStatic<T>;

        type RenderInit = RenderInit<'a, T>;

        type CachedIntoProvideValue = ProvideRef<'a, T>;
        fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
            ProvideRef(self)
        }

        fn match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_match(self)
        }

        fn not_match_cache(&self, cache: &Self::Cache) -> bool {
            cache.cache_not_match(self)
        }

        fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
            (PartialCacheCowStatic::from_ref(self), RenderInit(self))
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(TempRef<T>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            *cache = PartialCacheCowStatic::from_ref(self);
            renderer(TempRef(&self))
        }
    }
}
