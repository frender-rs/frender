use crate::{
    temp_str::{IntoStaticStr, IntoStaticStrCache, TempStrIntoStaticStrCache},
    TempStr, ToAsRefStr,
};

pub trait SsrStr {
    /// `'static` is actually not required for ssr.
    /// However, component_fn and RenderWith require the returned element to
    /// have the same `SsrElement::HtmlChildren` for all lifetime generics.
    /// Thus, we require `'static` in advance for earlier compile errors.
    type StaticStr: 'static + AsRef<str>;
    type IntoIntoStaticStr: IntoStaticStr<StaticStr = Self::StaticStr>;
    fn into_into_static_str(self) -> Self::IntoIntoStaticStr;
}

impl<S: 'static + AsRef<str>> SsrStr for S {
    type StaticStr = S;
    type IntoIntoStaticStr = SelfIntoStaticStr<S>;
    fn into_into_static_str(self) -> Self::IntoIntoStaticStr {
        SelfIntoStaticStr(self)
    }
}

impl<S: IntoStaticStr> SsrStr for TempStr<S> {
    type StaticStr = S::StaticStr;
    type IntoIntoStaticStr = S;

    fn into_into_static_str(self) -> Self::IntoIntoStaticStr {
        self.0
    }
}

pub trait CsrStr {
    type StaticStrCache: 'static
        + PartialEq<Self::IntoIntoStaticStrCache>
        + ToAsRefStr
        + PartialEq<Self>;
    type IntoIntoStaticStrCache: IntoStaticStrCache<StaticStrCache = Self::StaticStrCache>;
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache;
}

impl<S: 'static + AsRef<str> + PartialEq> CsrStr for S {
    type StaticStrCache = SelfToAsRefStr<S>;
    type IntoIntoStaticStrCache = SelfToAsRefStr<S>;
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache {
        SelfToAsRefStr(self)
    }
}

impl<S: IntoStaticStrCache> CsrStr for TempStr<S> {
    type StaticStrCache = TempStrIntoStaticStrCache<S::StaticStrCache>;
    type IntoIntoStaticStrCache = Self;
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache {
        self
    }
}

pub struct SelfIntoStaticStr<S>(pub S);

impl<S: 'static + AsRef<str>> IntoStaticStr for SelfIntoStaticStr<S> {
    type StaticStr = S;

    fn into_static_str(self) -> Self::StaticStr {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelfToAsRefStr<S>(pub S);

impl<S: PartialEq> PartialEq<S> for SelfToAsRefStr<S> {
    fn eq(&self, other: &S) -> bool {
        S::eq(&self.0, other)
    }

    fn ne(&self, other: &S) -> bool {
        S::ne(&self.0, other)
    }
}

impl<S: AsRef<str>> ToAsRefStr for SelfToAsRefStr<S> {
    type ToAsRefStr<'a> = &'a S
    where
        Self: 'a;

    fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
        &self.0
    }
}

impl<S: 'static + AsRef<str> + PartialEq> IntoStaticStrCache for SelfToAsRefStr<S> {
    type StaticStrCache = Self;

    fn into_static_str_cache(self) -> Self {
        self
    }

    fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
        *cache = self
    }
}

pub mod csr {
    use crate::{IntoStaticStrCache, ToAsRefStr};

    use super::CsrStr;

    pub fn update_with_option_cache<S: CsrStr, R>(
        s: S,
        cache: &mut Option<S::StaticStrCache>,
        update: impl FnOnce(&str) -> R,
    ) -> Option<R> {
        let cache = if let Some(cache) = cache {
            if *cache == s {
                return None;
            }

            s.into_into_static_str_cache()
                .update_into_static_str_cache(cache);

            cache
        } else {
            cache.insert(s.into_into_static_str_cache().into_static_str_cache())
        };

        Some(update(cache.to_as_ref_str().as_ref()))
    }

    pub fn init_cache<S: CsrStr, R>(
        //
        s: S,
        update: impl FnOnce(&str) -> R,
    ) -> (S::StaticStrCache, R) {
        let cache = s.into_into_static_str_cache().into_static_str_cache();

        let res = update(cache.to_as_ref_str().as_ref());
        (cache, res)
    }
}
