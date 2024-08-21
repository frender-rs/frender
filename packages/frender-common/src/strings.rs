use crate::{temp_str::IntoAsRefStr, TempStr, ToAsRefStr, ToStaticCache, ToStaticStr};

/// A trait alias for `'static + AsRef<str> + Clone`
pub trait StaticAsRefStrAndClone: 'static + AsRef<str> + Clone {}

impl<T: 'static + AsRef<str> + Clone> StaticAsRefStrAndClone for T {}

#[derive(Debug, Clone, Copy)]
pub struct SelfToStaticStrWithAsRefAndClone<S: StaticAsRefStrAndClone>(pub S);

impl<S: StaticAsRefStrAndClone> AsRef<str> for SelfToStaticStrWithAsRefAndClone<S> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub struct RefSelfToStaticStrWithAsRefAndClone<'a, S: StaticAsRefStrAndClone>(pub &'a S);

impl<'a, S: StaticAsRefStrAndClone> Clone for RefSelfToStaticStrWithAsRefAndClone<'a, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, S: StaticAsRefStrAndClone> Copy for RefSelfToStaticStrWithAsRefAndClone<'a, S> {}

impl<S: StaticAsRefStrAndClone> ToStaticStr for SelfToStaticStrWithAsRefAndClone<S> {
    type StaticStr = S;

    fn to_static_str(&self) -> Self::StaticStr {
        S::clone(&self.0)
    }

    fn into_static_str(self) -> Self::StaticStr {
        self.0
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        S::clone_from(target, &self.0)
    }
}

impl<S: StaticAsRefStrAndClone> ToAsRefStr for SelfToStaticStrWithAsRefAndClone<S> {
    type ToAsRefStr<'a> = RefSelfToStaticStrWithAsRefAndClone<'a, S>
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        RefSelfToStaticStrWithAsRefAndClone(&self.0)
    }
}

impl<S: StaticAsRefStrAndClone> AsRef<str> for RefSelfToStaticStrWithAsRefAndClone<'_, S> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a, S: StaticAsRefStrAndClone> ToStaticStr for RefSelfToStaticStrWithAsRefAndClone<'a, S> {
    type StaticStr = S;

    fn to_static_str(&self) -> Self::StaticStr {
        S::clone(self.0)
    }

    fn update_to_static_str(&self, target: &mut Self::StaticStr) {
        S::clone_from(target, self.0)
    }

    fn update_into_static_str(self, target: &mut Self::StaticStr) {
        self.update_to_static_str(target)
    }
}

impl<S: StaticAsRefStrAndClone> ToAsRefStr for RefSelfToStaticStrWithAsRefAndClone<'_, S> {
    type ToAsRefStr<'a> = Self
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        *self
    }
}

/// [`impl StaticAsRefStrAndClone`](StaticAsRefStrAndClone) |
/// <code>[TempStr]<impl [ToStaticStr](ToStaticStr)></code>
pub trait StrToStaticStr {
    type StaticStr: 'static + AsRef<str>;
    type IntoToStaticStr: ToStaticStr<StaticStr = Self::StaticStr>;
    type ToToStaticStr<'a>: ToStaticStr<StaticStr = Self::StaticStr>
    where
        Self: 'a;

    fn into_to_static_str(self) -> Self::IntoToStaticStr;
    fn to_to_static_str(&self) -> Self::ToToStaticStr<'_>;
}

impl<S: StaticAsRefStrAndClone> StrToStaticStr for S {
    type StaticStr = S;

    type IntoToStaticStr = SelfToStaticStrWithAsRefAndClone<S>;

    type ToToStaticStr<'a> = RefSelfToStaticStrWithAsRefAndClone<'a, S>
    where
        Self: 'a;

    fn into_to_static_str(self) -> Self::IntoToStaticStr {
        SelfToStaticStrWithAsRefAndClone(self)
    }

    fn to_to_static_str(&self) -> Self::ToToStaticStr<'_> {
        RefSelfToStaticStrWithAsRefAndClone(self)
    }
}

impl<S: ToStaticStr> StrToStaticStr for TempStr<S> {
    type StaticStr = S::StaticStr;
    type IntoToStaticStr = S;
    type ToToStaticStr<'a> = &'a S
    where
        Self: 'a;

    fn into_to_static_str(self) -> Self::IntoToStaticStr {
        self.0
    }

    fn to_to_static_str(&self) -> Self::ToToStaticStr<'_> {
        &self.0
    }
}

/// One of the following types:
///
/// - [`impl StaticAsRefStrAndClone`](StaticAsRefStrAndClone)
/// - <code> [TempStr]<impl [ToAsRefStr](ToAsRefStr)> </code>
pub trait StrToAsRefStr: StrToStaticStr {
    type IntoAsRefStr: AsRef<str> + ToStaticStr<StaticStr = Self::StaticStr>;

    type ToAsRefStr<'a>: AsRef<str> + ToStaticStr<StaticStr = Self::StaticStr>
    where
        Self: 'a;

    fn into_as_ref_str(self) -> Self::IntoAsRefStr;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_>;
}

impl<S: StaticAsRefStrAndClone> StrToAsRefStr for S {
    type IntoAsRefStr = SelfToStaticStrWithAsRefAndClone<S>;
    type ToAsRefStr<'a> = RefSelfToStaticStrWithAsRefAndClone<'a, S>
    where
        Self: 'a;

    fn into_as_ref_str(self) -> Self::IntoAsRefStr {
        SelfToStaticStrWithAsRefAndClone(self)
    }

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        RefSelfToStaticStrWithAsRefAndClone(self)
    }
}

impl<S: IntoAsRefStr> StrToAsRefStr for TempStr<S> {
    type IntoAsRefStr = S::IntoAsRefStr;

    type ToAsRefStr<'a> = S::ToAsRefStr<'a>
    where
        Self: 'a;

    fn into_as_ref_str(self) -> Self::IntoAsRefStr {
        self.0.into_as_ref_str()
    }

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        self.0.to_as_ref_str()
    }
}

/// A trait alias for <code>[StaticAsRefStrAndClone] + [PartialEq]</code>
pub trait StaticAsRefStrAndCloneAndPartialEq: StaticAsRefStrAndClone + PartialEq {}

impl<T: StaticAsRefStrAndClone + PartialEq> StaticAsRefStrAndCloneAndPartialEq for T {}

/// One of the following types:
///
/// - [`impl StaticAsRefStrAndCloneAndPartialEq`](StaticAsRefStrAndCloneAndPartialEq)
/// - <code>[TempStr]<impl [ToAsRefStr](ToAsRefStr)></code>
pub trait StrToStaticCache {
    type StaticCache: 'static;
    type IntoToStaticCache: ToAsRefStr + ToStaticCache<StaticCache = Self::StaticCache>;
    type ToToStaticCache<'a>: ToAsRefStr + ToStaticCache<StaticCache = Self::StaticCache>
    where
        Self: 'a;

    fn into_to_static_cache(self) -> Self::IntoToStaticCache;
    fn to_to_static_cache(&self) -> Self::ToToStaticCache<'_>;
}

impl<T: StaticAsRefStrAndCloneAndPartialEq> ToStaticCache for SelfToStaticStrWithAsRefAndClone<T> {
    type StaticCache = T;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool {
        self.0 == *cache
    }

    fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
        self.0 != *cache
    }

    fn to_static_cache(&self) -> Self::StaticCache {
        T::clone(&self.0)
    }

    fn into_static_cache(self) -> Self::StaticCache {
        self.0
    }

    fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
        T::clone_from(target, &self.0)
    }
}

impl<T: StaticAsRefStrAndCloneAndPartialEq> ToStaticCache
    for RefSelfToStaticStrWithAsRefAndClone<'_, T>
{
    type StaticCache = T;

    fn match_cache(&self, cache: &Self::StaticCache) -> bool {
        self.0 == cache
    }

    fn not_match_cache(&self, cache: &Self::StaticCache) -> bool {
        self.0 != cache
    }

    fn to_static_cache(&self) -> Self::StaticCache {
        T::clone(self.0)
    }

    fn update_to_static_cache(&self, target: &mut Self::StaticCache) {
        T::clone_from(target, &self.0)
    }

    fn update_into_static_cache(self, target: &mut Self::StaticCache) {
        self.update_to_static_cache(target)
    }
}

impl<S: StaticAsRefStrAndCloneAndPartialEq> StrToStaticCache for S {
    type StaticCache = S;
    type IntoToStaticCache = SelfToStaticStrWithAsRefAndClone<S>;

    type ToToStaticCache<'a> = RefSelfToStaticStrWithAsRefAndClone<'a, S>
    where
        Self: 'a;

    fn into_to_static_cache(self) -> Self::IntoToStaticCache {
        SelfToStaticStrWithAsRefAndClone(self)
    }

    fn to_to_static_cache(&self) -> Self::ToToStaticCache<'_> {
        RefSelfToStaticStrWithAsRefAndClone(self)
    }
}

impl<S: ToStaticCache + ToAsRefStr> StrToStaticCache for TempStr<S> {
    type StaticCache = S::StaticCache;
    type IntoToStaticCache = S;
    type ToToStaticCache<'a> = &'a S
    where
        Self: 'a;

    fn into_to_static_cache(self) -> Self::IntoToStaticCache {
        self.0
    }

    fn to_to_static_cache(&self) -> Self::ToToStaticCache<'_> {
        &self.0
    }
}
