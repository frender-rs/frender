use std::borrow::{Borrow, Cow};

use crate::static_or_temp_ref::StaticOrTempRef;

use super::{cheap_clone_pointer::KnownCheapClonePointer, IntoStatic, ToStatic};

/// This trait should only be implemented for types that may not be `'static`.
pub trait IntoStaticCache<V: ?Sized + 'static>: IntoStatic<V> {
    type IntoStaticCache: 'static + Borrow<V>;

    fn match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool;
    fn not_match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        !self.match_into_static_cache(cache)
    }

    fn into_static_str_cache(self) -> Self::IntoStaticCache;
    fn update_into_static_str_cache(self, cache: &mut Self::IntoStaticCache)
    where
        Self: Sized,
    {
        *cache = self.into_static_str_cache();
    }
}

pub trait ToStaticCache<V: ?Sized + 'static>: ToStatic<V> {
    type ToStaticCache: 'static + Borrow<V>;

    fn match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool;
    fn not_match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        !self.match_to_static_cache(cache)
    }

    fn to_static_str_cache(&self) -> Self::ToStaticCache;
    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticCache) {
        *cache = self.to_static_str_cache();
    }
}

impl<T: ?Sized + ToStaticCache<V>, V: ?Sized + 'static> IntoStaticCache<V> for &T {
    type IntoStaticCache = T::ToStaticCache;

    fn match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        T::match_to_static_cache(self, cache)
    }

    fn not_match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        T::not_match_to_static_cache(self, cache)
    }

    fn into_static_str_cache(self) -> Self::IntoStaticCache {
        T::to_static_str_cache(self)
    }

    fn update_into_static_str_cache(self, cache: &mut Self::IntoStaticCache)
    where
        Self: Sized,
    {
        T::update_to_static_str_cache(self, cache)
    }
}

impl<V: ?Sized + 'static + ToOwned + PartialEq> ToStaticCache<V> for V {
    type ToStaticCache = V::Owned;

    fn match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        V::eq(self, cache.borrow())
    }

    fn not_match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        V::ne(self, cache.borrow())
    }

    fn to_static_str_cache(&self) -> Self::ToStaticCache {
        V::to_owned(self)
    }

    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticCache) {
        V::clone_into(self, cache)
    }
}

impl<V: ?Sized + 'static + ToOwned + PartialEq> ToStaticCache<V> for Cow<'static, V> {
    type ToStaticCache = Self;

    fn match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        Self::eq(self, cache)
    }

    fn not_match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        Self::ne(self, cache)
    }

    fn to_static_str_cache(&self) -> Self::ToStaticCache {
        Self::clone(self)
    }

    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticCache) {
        Self::clone_from(cache, self)
    }
}

impl<T: KnownCheapClonePointer<Target = str>> ToStaticCache<str> for T {
    type ToStaticCache = T;

    fn match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        Self::eq(self, cache)
    }

    fn not_match_to_static_cache(&self, cache: &Self::ToStaticCache) -> bool {
        Self::ne(self, cache)
    }

    fn to_static_str_cache(&self) -> Self::ToStaticCache {
        Self::clone(self)
    }

    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticCache) {
        cache.clone_from(self)
    }
}

impl<V: ?Sized + 'static + ToOwned + PartialEq> IntoStaticCache<V> for Cow<'_, V> {
    type IntoStaticCache = V::Owned;

    fn match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        V::eq(self, cache.borrow())
    }

    fn not_match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        V::ne(self, cache.borrow())
    }

    fn into_static_str_cache(self) -> Self::IntoStaticCache {
        self.into_owned()
    }

    fn update_into_static_str_cache(self, cache: &mut Self::IntoStaticCache) {
        match self {
            Cow::Borrowed(this) => this.clone_into(cache),
            Cow::Owned(this) => *cache = this,
        }
    }
}

impl<V: ?Sized + 'static + ToOwned + PartialEq> IntoStaticCache<V> for StaticOrTempRef<'_, V> {
    type IntoStaticCache = Cow<'static, V>;

    fn match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        *self == *cache
    }

    fn not_match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        *self != *cache
    }

    fn into_static_str_cache(self) -> Self::IntoStaticCache {
        self.to_owned_cow_static()
    }

    fn update_into_static_str_cache(self, cache: &mut Self::IntoStaticCache) {
        self.clone_into_cow_static(cache)
    }
}

impl<V: ?Sized + 'static + ToOwned + PartialEq> IntoStaticCache<Cow<'static, V>>
    for StaticOrTempRef<'_, V>
{
    type IntoStaticCache = Cow<'static, V>;

    fn match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        *self == *cache
    }

    fn not_match_into_static_cache(&self, cache: &Self::IntoStaticCache) -> bool {
        *self != *cache
    }

    fn into_static_str_cache(self) -> Self::IntoStaticCache {
        self.to_owned_cow_static()
    }

    fn update_into_static_str_cache(self, cache: &mut Self::IntoStaticCache) {
        self.clone_into_cow_static(cache)
    }
}
