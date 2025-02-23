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

use crate::value_kind::KindOfStaticRefOrTempOwned;

pub type Kind<T> = KindOfStaticRefOrTempOwned<T>;

pub mod refed {
    use std::borrow::Cow;

    use crate::{
        non_reactive::{CachedNonReactiveValue, CachedNonReactiveValueRenderInit},
        value_kind::StaticRefOrTempOwned,
        ProvideValueOfKind,
    };

    use super::Kind;

    pub struct Provide<T: ?Sized + 'static + ToOwned>(Cow<'static, T>);

    impl<T: ?Sized + 'static + ToOwned> ProvideValueOfKind<Kind<T>> for Provide<T> {
        fn provide_value_of_kind<Out>(
            self,
            f: impl FnOnce(StaticRefOrTempOwned<'_, T>) -> Out,
        ) -> Out {
            f(From::from(&self.0))
        }
    }

    pub struct RenderInit;

    impl<T: ?Sized + 'static + ToOwned + PartialEq>
        CachedNonReactiveValueRenderInit<Kind<T>, Cow<'static, T>> for RenderInit
    {
        fn cached_non_reactive_value_render_init<Out>(
            self,
            renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            cache: &mut Cow<'static, T>,
        ) -> Out {
            renderer(From::<&_>::from(cache))
        }
    }

    impl<T: ?Sized + 'static + ToOwned + PartialEq> CachedNonReactiveValue<Kind<T>>
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
            renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            *cache = self;
            renderer(From::<&_>::from(cache))
        }
    }
}

pub mod r#ref {
    pub use super::static_or_temp::RenderInit;

    use std::borrow::Cow;

    use crate::{
        non_reactive::{CachedNonReactiveValue, CloneIfCacheMiss, UncachedNonReactiveValue},
        value_kind::StaticRefOrTempOwned,
        ProvideValueOfKind,
    };

    use super::Kind;

    pub struct Provide<'a, T: ?Sized + 'static + ToOwned>(&'a Cow<'static, T>);

    impl<T: ?Sized + 'static + ToOwned> ProvideValueOfKind<Kind<T>> for Provide<'_, T> {
        fn provide_value_of_kind<Out>(
            self,
            f: impl FnOnce(StaticRefOrTempOwned<'_, T>) -> Out,
        ) -> Out {
            f(From::from(self.0))
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned> UncachedNonReactiveValue<Kind<T>> for &'a Cow<'static, T> {
        type UncachedIntoProvideValue = Provide<'a, T>;

        fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
            Provide(self)
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned + PartialEq> CachedNonReactiveValue<Kind<T>>
        for CloneIfCacheMiss<&'a Cow<'static, T>>
    {
        type CacheCanProvideValue = super::super::CacheCanProvideValue;
        type Cache = Cow<'static, T>;

        type RenderInit = RenderInit<'a, T>;

        type CachedIntoProvideValue = Provide<'a, T>;
        fn cached_into_provide_value(self) -> Self::CachedIntoProvideValue {
            Provide(self.0)
        }

        fn match_cache(&self, cache: &Self::Cache) -> bool {
            <Cow<'static, T>>::eq(self.0, cache)
        }

        fn not_match_cache(&self, cache: &Self::Cache) -> bool {
            <Cow<'static, T>>::ne(self.0, cache)
        }

        fn into_cache_and_render_init(self) -> (Self::Cache, Self::RenderInit) {
            (self.0.clone(), RenderInit(From::from(self.0)))
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            cache.clone_from(self.0);
            renderer(From::from(self.0))
        }
    }
}

pub mod static_or_temp {
    use std::borrow::Cow;

    use crate::{
        non_reactive::{
            CachedNonReactiveValue, CachedNonReactiveValueRenderInit, UncachedNonReactiveValue,
        },
        value_kind::StaticRefOrTempOwned,
        ProvideValueOfKind,
    };

    use super::Kind;

    pub struct Provide<'a, T: ?Sized + 'static + ToOwned>(StaticRefOrTempOwned<'a, T>);

    impl<T: ?Sized + 'static + ToOwned> ProvideValueOfKind<Kind<T>> for Provide<'_, T> {
        fn provide_value_of_kind<Out>(
            self,
            f: impl FnOnce(StaticRefOrTempOwned<'_, T>) -> Out,
        ) -> Out {
            f(self.0)
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned> UncachedNonReactiveValue<Kind<T>>
        for StaticRefOrTempOwned<'a, T>
    {
        type UncachedIntoProvideValue = Provide<'a, T>;

        fn uncached_into_provide_value(self) -> Self::UncachedIntoProvideValue {
            Provide(self)
        }
    }

    pub struct RenderInit<'a, T: ?Sized + 'static + ToOwned>(pub StaticRefOrTempOwned<'a, T>);

    impl<'a, T: ?Sized + 'static + ToOwned>
        CachedNonReactiveValueRenderInit<Kind<T>, Cow<'static, T>> for RenderInit<'a, T>
    {
        fn cached_non_reactive_value_render_init<Out>(
            self,
            renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            _: &mut Cow<'static, T>,
        ) -> Out {
            renderer(self.0)
        }
    }

    impl<'a, T: ?Sized + 'static + ToOwned + PartialEq> CachedNonReactiveValue<Kind<T>>
        for StaticRefOrTempOwned<'a, T>
    {
        type CacheCanProvideValue = super::super::CacheCanProvideValue;
        type Cache = Cow<'static, T>;

        type RenderInit = RenderInit<'a, T>;

        type CachedIntoProvideValue = Provide<'a, T>;
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
            (self.to_owned_cow_static(), RenderInit(self))
        }

        fn update_into_cache_and_render<Out>(
            self,
            renderer: impl FnOnce(<Kind<T> as crate::value_kind::ValueKind>::Value<'_>) -> Out,
            cache: &mut Self::Cache,
        ) -> Out {
            self.clone_into_cow_static(cache);
            renderer(self)
        }
    }
}
