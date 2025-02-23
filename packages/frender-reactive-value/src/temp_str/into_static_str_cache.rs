use frender_common::impl_many;

/// This trait should only be implemented for types that may not be `'static`.
pub trait IntoStaticStrCache {
    type StaticStrCache: 'static + PartialEq<Self> + AsRef<str>;

    fn into_static_str_cache(self) -> Self::StaticStrCache;
    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache)
    where
        Self: Sized,
    {
        *cache = self.into_static_str_cache();
    }
}

pub trait ToStaticStrCache {
    type ToStaticStrCache: 'static + PartialEq<Self> + AsRef<str>;
    fn to_static_str_cache(&self) -> Self::ToStaticStrCache;
    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticStrCache) {
        *cache = self.to_static_str_cache();
    }
}

impl<T: ?Sized + ToStaticStrCache> IntoStaticStrCache for &T {
    type StaticStrCache = RefToStaticStrCache<T::ToStaticStrCache>;

    fn into_static_str_cache(self) -> Self::StaticStrCache {
        RefToStaticStrCache(T::to_static_str_cache(self))
    }
}

pub struct RefToStaticStrCache<S>(pub S);

impl<S: AsRef<str>> AsRef<str> for RefToStaticStrCache<S> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<S: PartialEq<T>, T: ?Sized> PartialEq<&T> for RefToStaticStrCache<S> {
    fn eq(&self, other: &&T) -> bool {
        S::eq(&self.0, other)
    }

    fn ne(&self, other: &&T) -> bool {
        S::ne(&self.0, other)
    }
}

impl ToStaticStrCache for str {
    type ToStaticStrCache = String;

    fn to_static_str_cache(&self) -> Self::ToStaticStrCache {
        self.to_owned()
    }

    fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticStrCache) {
        self.clone_into(cache);
    }
}

impl_many!(
    impl<__> ToStaticStrCache
        for each_of![
            //
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type ToStaticStrCache = Self;

        fn to_static_str_cache(&self) -> Self::ToStaticStrCache {
            self.clone()
        }

        fn update_to_static_str_cache(&self, cache: &mut Self::ToStaticStrCache) {
            Self::clone_from(cache, self);
        }
    }
);

impl IntoStaticStrCache for std::borrow::Cow<'_, str> {
    type StaticStrCache = String;

    fn into_static_str_cache(self) -> Self::StaticStrCache {
        self.into_owned()
    }

    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache)
    where
        Self: Sized,
    {
        match self {
            std::borrow::Cow::Borrowed(this) => this.clone_into(cache),
            std::borrow::Cow::Owned(this) => *cache = this,
        }
    }
}
