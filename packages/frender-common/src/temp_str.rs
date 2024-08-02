use std::borrow::Borrow;

#[derive(Debug, Clone, Copy)]
pub struct TempStr<S>(pub S);

pub trait ToStaticStr {
    type StaticStr: 'static + Borrow<str>;

    fn to_static_str(&self) -> Self::StaticStr;
    fn into_static_str(self) -> Self::StaticStr
    where
        Self: Sized,
    {
        self.to_static_str()
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        *target = self.to_static_str()
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr)
    where
        Self: Sized,
    {
        *target = self.into_static_str()
    }
}

impl<T: ToStaticStr + ?Sized> ToStaticStr for &T {
    type StaticStr = T::StaticStr;

    fn to_static_str(&self) -> Self::StaticStr {
        T::to_static_str(self)
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr)
    where
        Self: Sized,
    {
        T::update_to_static_str(self, target)
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr)
    where
        Self: Sized,
    {
        T::update_to_static_str(self, target)
    }
}

impl ToStaticStr for str {
    type StaticStr = String;

    fn to_static_str(&self) -> Self::StaticStr {
        self.to_owned()
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        self.clone_into(target)
    }
}

impl ToStaticStr for String {
    type StaticStr = String;

    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }

    fn into_static_str(self) -> Self::StaticStr {
        self
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        self.clone_into(target)
    }
}

impl ToStaticStr for std::borrow::Cow<'_, str> {
    type StaticStr = String;

    fn to_static_str(&self) -> Self::StaticStr {
        match self {
            std::borrow::Cow::Borrowed(s) => str::to_owned(s),
            std::borrow::Cow::Owned(s) => s.clone(),
        }
    }

    fn into_static_str(self) -> Self::StaticStr {
        self.into_owned()
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        match self {
            std::borrow::Cow::Borrowed(s) => str::update_to_static_str(s, target),
            std::borrow::Cow::Owned(s) => String::update_to_static_str(s, target),
        }
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr) {
        match self {
            std::borrow::Cow::Borrowed(v) => v.clone_into(target),
            std::borrow::Cow::Owned(v) => *target = v,
        }
    }
}

impl ToStaticStr for std::rc::Rc<str> {
    type StaticStr = Self;

    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl ToStaticStr for std::sync::Arc<str> {
    type StaticStr = Self;

    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

pub trait ToStaticCache {
    type StaticCache: 'static;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool;
    fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
        !self.match_cache(cache)
    }

    fn to_static_cache(&self) -> Self::StaticCache;

    fn into_static_cache(self) -> Self::StaticCache
    where
        Self: Sized,
    {
        self.to_static_cache()
    }

    fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
        *target = self.to_static_cache()
    }

    fn update_into_static_cache(self, target: &mut Self::StaticCache)
    where
        Self: Sized,
    {
        *target = self.into_static_cache()
    }
}

impl<T: ?Sized + ToStaticCache> ToStaticCache for &T {
    type StaticCache = T::StaticCache;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool {
        T::match_cache(self, cache)
    }

    fn to_static_cache(&self) -> Self::StaticCache {
        T::to_static_cache(self)
    }

    fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
        T::not_match_cache(self, cache)
    }

    fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
        T::update_to_static_cache(self, target)
    }

    fn update_into_static_cache(self, target: &mut Self::StaticCache)
    where
        Self: Sized,
    {
        T::update_to_static_cache(self, target)
    }
}

impl ToStaticCache for str {
    type StaticCache = <Self as ToStaticStr>::StaticStr;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool {
        *self == *cache
    }

    fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
        *self != *cache
    }

    fn to_static_cache(&self) -> Self::StaticCache {
        self.to_static_str()
    }

    fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
        self.update_to_static_str(target)
    }
}

crate::impl_many!(
    impl<__> ToStaticCache
        for each_of![
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type StaticCache = <Self as ToStaticStr>::StaticStr;

        fn match_cache(&self, cache: &Self::StaticCache) -> bool {
            *self == *cache
        }

        fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
            *self != *cache
        }

        fn to_static_cache(&self) -> Self::StaticCache {
            self.to_static_str()
        }

        fn into_static_cache(self) -> Self::StaticCache {
            self.into_static_str()
        }

        fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
            self.update_to_static_str(target)
        }

        fn update_into_static_cache(self, target: &mut Self::StaticCache) {
            self.update_into_static_str(target)
        }
    }
);

/// Consider this trait as a borrowed version of [`ToString`].
pub trait ToAsRefStr: ToStaticStr {
    type ToAsRefStr<'a>: AsRef<str> + ToStaticStr
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_>;
}

impl<'this, T: ?Sized + ToAsRefStr> ToAsRefStr for &'this T {
    type ToAsRefStr<'a> = T::ToAsRefStr<'this>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        T::to_as_ref_str(self)
    }
}

impl ToAsRefStr for str {
    type ToAsRefStr<'a> = &'a str
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        self
    }
}

crate::impl_many!(
    impl<__> ToAsRefStr
        for each_of![
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type ToAsRefStr<'a> = &'a Self
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self
        }
    }
);

#[cfg(test)]
mod asserts {
    use super::{ToAsRefStr, ToStaticCache, ToStaticStr};

    #[test]
    const fn test<'a>()
    where
        &'a str: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
        &'a String: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
        std::borrow::Cow<'a, str>: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
        &'a std::borrow::Cow<'a, str>: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
        &'a std::rc::Rc<str>: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
        &'a std::sync::Arc<str>: AsRef<str> + ToStaticStr + ToStaticCache + ToAsRefStr,
    {
    }

    const _: () = test();
}
