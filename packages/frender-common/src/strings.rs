use crate::{
    temp_str::{IntoStaticStr, IntoStaticStrCache, TempStrIntoStaticStrCache},
    TempStr, ToAsRefStr,
};

pub mod csr;

pub trait IsNonReactiveStr {
    fn into_reactive_value(self) -> NonReactiveStr<Self>
    where
        Self: Sized,
    {
        NonReactiveStr(self)
    }
}

pub trait SsrStr: IsNonReactiveStr {
    /// `'static` is actually not required for ssr.
    /// However, component_fn and RenderWith require the returned element to
    /// have the same `SsrElement::HtmlChildren` for all lifetime generics.
    /// Thus, we require `'static` in advance for earlier compile errors.
    type StaticStr: 'static + AsRef<str>;
    type IntoIntoStaticStr: IntoStaticStr<StaticStr = Self::StaticStr>;
    fn into_into_static_str(self) -> Self::IntoIntoStaticStr;
}

pub trait CsrStr: IsNonReactiveStr {
    type StaticStrCache: 'static + PartialEq<Self::IntoIntoStaticStrCache> + ToAsRefStr;
    type IntoIntoStaticStrCache: IntoStaticStrCache<StaticStrCache = Self::StaticStrCache>;
    /// The implementation should be zero cost.
    /// Costs should be put into `impl IntoStaticStrCache`
    /// so that existing `Self::StaticStrCache` can check whether the cache matches `Self::IntoIntoStaticStrCache`
    /// before calling [`IntoStaticStrCache::update_into_static_str_cache`].
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache;

    fn match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool;

    fn not_match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        !self.match_static_str_cache(cache)
    }
}

// region: static strings
impl<S: 'static + AsRef<str>> IsNonReactiveStr for S {}
impl<S: 'static + AsRef<str>> SsrStr for S {
    type StaticStr = S;
    type IntoIntoStaticStr = SelfIntoStaticStr<S>;
    fn into_into_static_str(self) -> Self::IntoIntoStaticStr {
        SelfIntoStaticStr(self)
    }
}
impl<S: 'static + AsRef<str> + PartialEq> CsrStr for S {
    type StaticStrCache = SelfToAsRefStr<S>;
    type IntoIntoStaticStrCache = SelfToAsRefStr<S>;
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache {
        SelfToAsRefStr(self)
    }
    fn match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        *self == cache.0
    }
    fn not_match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        *self != cache.0
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

impl<S: AsRef<str>> ToAsRefStr for SelfToAsRefStr<S> {
    type ToAsRefStr<'a>
        = &'a S
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

// endregion
// region: TempStr
impl<S> IsNonReactiveStr for TempStr<S> {}
impl<S: IntoStaticStr> SsrStr for TempStr<S> {
    type StaticStr = S::StaticStr;
    type IntoIntoStaticStr = S;

    fn into_into_static_str(self) -> Self::IntoIntoStaticStr {
        self.0
    }
}
impl<S: IntoStaticStrCache> CsrStr for TempStr<S> {
    type StaticStrCache = TempStrIntoStaticStrCache<S::StaticStrCache>;
    type IntoIntoStaticStrCache = Self;
    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache {
        self
    }
    fn match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        cache == self
    }
    fn not_match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        cache != self
    }
}
// endregion
// region: NonReactiveStr
pub struct NonReactiveStr<T: IsNonReactiveStr>(pub T);

impl<T: IsNonReactiveStr> IsNonReactiveStr for NonReactiveStr<T> {}

impl<T: SsrStr> SsrStr for NonReactiveStr<T> {
    type StaticStr = T::StaticStr;
    type IntoIntoStaticStr = T::IntoIntoStaticStr;

    fn into_into_static_str(self) -> Self::IntoIntoStaticStr {
        self.0.into_into_static_str()
    }
}

impl<T: CsrStr> CsrStr for NonReactiveStr<T> {
    type StaticStrCache = T::StaticStrCache;
    type IntoIntoStaticStrCache = T::IntoIntoStaticStrCache;

    fn into_into_static_str_cache(self) -> Self::IntoIntoStaticStrCache {
        self.0.into_into_static_str_cache()
    }

    fn match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        self.0.match_static_str_cache(cache)
    }

    fn not_match_static_str_cache(&self, cache: &Self::StaticStrCache) -> bool {
        self.0.not_match_static_str_cache(cache)
    }
}
// endregion

pub mod define_trait_known_str {
    #[macro_export]
    #[doc(hidden)]
    macro_rules! __define_trait_known_str_impl_static {
        ($KnownStaticStr:ident) => {
            $crate::impl_many!(
                impl<__> $KnownStaticStr
                    for each_of![
                        &'static str,
                        String,
                        std::borrow::Cow<'static, str>,
                        std::rc::Rc<str>,
                        std::sync::Arc<str>,
                    ]
                {
                }
            );
        };
    }
}

/// We cannot ensure `Option<_>: !CsrStr` because
/// `core` might `impl AsRef<str> for Option<_>`.
/// But we can ensure `Option<_>: !KnownCsrStr`.
#[macro_export]
macro_rules! define_trait_known_str {
    (
        $(
            pub(crate) trait IsNonReactiveStr = $KnownIsNonReactiveStr:ident;
        )?
        pub(crate) trait Ssr = $KnownSsrStr:ident;
        pub(crate) trait Csr = $KnownCsrStr:ident;
    ) => {
        $(
            pub(crate) trait $KnownIsNonReactiveStr: $crate::strings::IsNonReactiveStr {}
        )?
        pub(crate) trait $KnownSsrStr: $crate::strings::SsrStr $(+ $KnownIsNonReactiveStr)? {}
        pub(crate) trait $KnownCsrStr: $crate::strings::CsrStr $(+ $KnownIsNonReactiveStr)? {}

        const _: () = {
            use std::convert::AsRef;

            use $crate::{
                strings::{IsNonReactiveStr, CsrStr, SsrStr},
                IntoStaticStr, IntoStaticStrCache,
            };

            trait KnownStaticStr: 'static + AsRef<str> + SsrStr + CsrStr {}

            $crate::__define_trait_known_str_impl_static! {KnownStaticStr}

            $(
                impl<S: KnownStaticStr> $KnownIsNonReactiveStr for S {}
            )?
            impl<S: KnownStaticStr> $KnownSsrStr for S {}
            impl<S: KnownStaticStr> $KnownCsrStr for S {}

            $(
                impl<S> $KnownIsNonReactiveStr for $crate::TempStr<S> {}
            )?
            impl<S: IntoStaticStr> $KnownSsrStr for $crate::TempStr<S> {}
            impl<S: IntoStaticStrCache> $KnownCsrStr for $crate::TempStr<S> {}

            $(
                impl<S: IsNonReactiveStr> $KnownIsNonReactiveStr for $crate::strings::NonReactiveStr<S> {}
            )?
            impl<S: SsrStr> $KnownSsrStr for $crate::strings::NonReactiveStr<S> {}
            impl<S: CsrStr> $KnownCsrStr for $crate::strings::NonReactiveStr<S> {}
        };
    };
}

/// This trait exists because we cannot `impl AsRef<str> for TempStr<_>`.
pub trait AsRefStr {
    fn as_ref_str(&self) -> &str;
}

impl<T: AsRef<str>> AsRefStr for T {
    fn as_ref_str(&self) -> &str {
        self.as_ref()
    }
}

impl<S: AsRefStr> AsRefStr for TempStr<S> {
    fn as_ref_str(&self) -> &str {
        self.0.as_ref_str()
    }
}
