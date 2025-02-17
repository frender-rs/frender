#[derive(Debug, Clone, Copy)]
pub struct TempStr<S>(pub S);

pub struct TempStrIntoStaticStrCache<C>(pub C);

impl<C: PartialEq<S>, S> PartialEq<TempStr<S>> for TempStrIntoStaticStrCache<C> {
    fn eq(&self, other: &TempStr<S>) -> bool {
        self.0 == other.0
    }
    fn ne(&self, other: &TempStr<S>) -> bool {
        self.0 != other.0
    }
}

impl<C: ToAsRefStr> ToAsRefStr for TempStrIntoStaticStrCache<C> {
    type ToAsRefStr<'a>
        = C::ToAsRefStr<'a>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        self.0.to_as_ref_str()
    }
}

// impl<S: ToAsRefStr> ToAsRefStr for TempStr<S> {
//     type ToAsRefStr<'a> = S::ToAsRefStr<'a>
//     where
//         Self: 'a;

//     fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
//         self.0.to_as_ref_str()
//     }
// }

impl<S: IntoStaticStrCache> IntoStaticStrCache for TempStr<S> {
    type StaticStrCache = TempStrIntoStaticStrCache<S::StaticStrCache>;

    fn into_static_str_cache(self) -> Self::StaticStrCache {
        TempStrIntoStaticStrCache(self.0.into_static_str_cache())
    }

    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
        S::update_into_static_str_cache(self.0, &mut cache.0)
    }
}

pub trait IntoStaticStr {
    type StaticStr: 'static + AsRef<str>;
    fn into_static_str(self) -> Self::StaticStr
    where
        Self: Sized;

    fn update_into_static_str(self, target: &mut Self::StaticStr)
    where
        Self: Sized,
    {
        *target = self.into_static_str()
    }
}

impl<T: ?Sized + ToStaticStr> IntoStaticStr for &T {
    type StaticStr = T::StaticStr;

    fn into_static_str(self) -> Self::StaticStr {
        self.to_static_str()
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr) {
        self.update_to_static_str(target)
    }
}

pub trait ToStaticStr: IntoStaticStr {
    fn to_static_str(&self) -> Self::StaticStr;

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        *target = self.to_static_str()
    }
}

impl<T: ToStaticStr + ?Sized> ToStaticStr for &T {
    fn to_static_str(&self) -> Self::StaticStr {
        T::to_static_str(self)
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        T::update_to_static_str(self, target)
    }
}

impl IntoStaticStr for &str {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self.to_owned()
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr) {
        self.clone_into(target)
    }
}

impl ToStaticStr for &str {
    fn to_static_str(&self) -> Self::StaticStr {
        self.into_static_str()
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        self.update_into_static_str(target)
    }
}

impl IntoStaticStr for String {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl ToStaticStr for String {
    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        self.clone_into(target)
    }
}

impl IntoStaticStr for std::borrow::Cow<'_, str> {
    type StaticStr = String;

    fn into_static_str(self) -> Self::StaticStr {
        self.into_owned()
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr) {
        match self {
            std::borrow::Cow::Borrowed(v) => v.clone_into(target),
            std::borrow::Cow::Owned(v) => *target = v,
        }
    }
}

impl ToStaticStr for std::borrow::Cow<'_, str> {
    fn to_static_str(&self) -> Self::StaticStr {
        match self {
            std::borrow::Cow::Borrowed(s) => str::to_owned(s),
            std::borrow::Cow::Owned(s) => s.clone(),
        }
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        match self {
            std::borrow::Cow::Borrowed(s) => <&str>::update_to_static_str(s, target),
            std::borrow::Cow::Owned(s) => String::update_to_static_str(s, target),
        }
    }
}

impl IntoStaticStr for std::rc::Rc<str> {
    type StaticStr = Self;

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl ToStaticStr for std::rc::Rc<str> {
    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }
}

impl IntoStaticStr for std::sync::Arc<str> {
    type StaticStr = Self;

    fn into_static_str(self) -> Self::StaticStr {
        self
    }
}

impl ToStaticStr for std::sync::Arc<str> {
    fn to_static_str(&self) -> Self::StaticStr {
        self.clone()
    }
}

pub trait IntoStaticStrCache {
    type StaticStrCache: 'static + PartialEq<Self> + ToAsRefStr;

    fn into_static_str_cache(self) -> Self::StaticStrCache;
    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache)
    where
        Self: Sized,
    {
        *cache = self.into_static_str_cache();
    }
}

pub trait ToStaticStrCache: IntoStaticStrCache {
    fn to_static_str_cache(&self) -> Self::StaticStrCache;
    fn update_to_static_str_cache(&self, cache: &mut Self::StaticStrCache) {
        *cache = self.to_static_str_cache();
    }
}

pub struct RefToStaticStrCache<S>(pub S);

impl<S: PartialEq<T>, T: ?Sized> PartialEq<&T> for RefToStaticStrCache<S> {
    fn eq(&self, other: &&T) -> bool {
        S::eq(&self.0, other)
    }

    fn ne(&self, other: &&T) -> bool {
        S::ne(&self.0, other)
    }
}

impl<S: ToAsRefStr> ToAsRefStr for RefToStaticStrCache<S> {
    type ToAsRefStr<'a>
        = S::ToAsRefStr<'a>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        self.0.to_as_ref_str()
    }
}

impl<S: ?Sized + ToStaticStrCache> IntoStaticStrCache for &S {
    type StaticStrCache = RefToStaticStrCache<S::StaticStrCache>;

    fn into_static_str_cache(self) -> Self::StaticStrCache {
        RefToStaticStrCache(S::to_static_str_cache(self))
    }

    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
        S::update_to_static_str_cache(self, &mut cache.0)
    }
}

impl<S: ?Sized + ToStaticStrCache> ToStaticStrCache for &S {
    fn to_static_str_cache(&self) -> Self::StaticStrCache {
        Self::into_static_str_cache(self)
    }

    fn update_to_static_str_cache(&self, cache: &mut Self::StaticStrCache) {
        Self::update_into_static_str_cache(self, cache)
    }
}

crate::impl_many!(
    impl<__> IntoStaticStrCache
        for each_of![
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type StaticStrCache = <Self as IntoStaticStr>::StaticStr;

        fn into_static_str_cache(self) -> Self::StaticStrCache {
            self.into_static_str()
        }

        fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
            self.update_into_static_str(cache)
        }
    }
);

crate::impl_many!(
    impl<__> ToStaticStrCache
        for each_of![
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        fn to_static_str_cache(&self) -> Self::StaticStrCache {
            self.to_static_str()
        }

        fn update_to_static_str_cache(&self, cache: &mut Self::StaticStrCache) {
            self.update_to_static_str(cache)
        }
    }
);

/// Consider this trait as a borrowed version of [`ToString`].
pub trait ToAsRefStr {
    type ToAsRefStr<'a>: AsRef<str>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_>;
}

impl<'this, T: ?Sized + ToAsRefStr> ToAsRefStr for &'this T {
    type ToAsRefStr<'a>
        = T::ToAsRefStr<'this>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        T::to_as_ref_str(self)
    }
}

crate::impl_many!(
    impl<__> ToAsRefStr
        for each_of![
            str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type ToAsRefStr<'a>
            = &'a Self
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self
        }
    }
);

#[cfg(test)]
mod asserts {
    use super::{IntoStaticStrCache, ToAsRefStr, ToStaticStr};

    #[test]
    const fn test<'a>()
    where
        &'a str: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
        &'a String: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
        std::borrow::Cow<'a, str>: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
        &'a std::borrow::Cow<'a, str>: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
        &'a std::rc::Rc<str>: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
        &'a std::sync::Arc<str>: AsRef<str> + ToStaticStr + IntoStaticStrCache + ToAsRefStr,
    {
    }

    const _: () = test();
}
