use ccss::token::tokens::IdentToken;
use ssr::IntoSsrDeclarationName;

/// A valid declaration name.
///
/// See https://drafts.csswg.org/css-syntax-3/#consume-declaration.
#[derive(Debug, Clone, Copy)]
pub struct DeclarationName<N>(N);

impl<'a> DeclarationName<&'a str> {
    /// Panics in [`IdentToken::new_const`] if s is invalid.
    pub const fn new_const(s: &'a str) -> Self {
        Self::from_parsed(IdentToken::new_const(s))
    }

    pub const fn from_parsed(s: IdentToken<'a>) -> Self {
        Self(s.original_str())
    }
}

impl<N> DeclarationName<N> {
    pub const fn unparsed(&self) -> &N {
        &self.0
    }

    pub fn into_unparsed(self) -> N {
        self.0
    }

    /// Panics if `name.as_ref()` is not a valid unparsed declaration name.
    pub fn new(name: N) -> Self
    where
        N: AsRef<str>,
    {
        DeclarationName::<&str>::new_const(name.as_ref());
        Self(name)
    }

    /// This relies on safe AsRef implementation
    pub(crate) fn as_ref_str(&self) -> DeclarationName<&str>
    where
        N: AsRef<str>,
    {
        DeclarationName(self.0.as_ref())
    }
}

/// Ssr DeclarationName
pub trait IntoDeclarationName: IntoSsrDeclarationName + csr::CsrDeclarationName {}

impl<S: ?Sized + IntoSsrDeclarationName + csr::CsrDeclarationName> IntoDeclarationName for S {}

pub mod ssr {

    use frender_common::{strings::SsrStr, IntoStaticStr};

    use super::DeclarationName;

    pub trait IntoSsrDeclarationName {
        type StaticDeclarationNameStr: 'static + AsRef<str>;

        fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr>;
    }

    impl<S: SsrStr> IntoSsrDeclarationName for S {
        type StaticDeclarationNameStr = S::StaticStr;

        fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr> {
            DeclarationName::new(self.into_into_static_str().into_static_str())
        }
    }

    impl<N: SsrStr> IntoSsrDeclarationName for DeclarationName<N> {
        type StaticDeclarationNameStr = N::StaticStr;

        fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr> {
            // This assumes StrToAsRefStr and ToStaticStr are implemented in the way that the string value doesn't change
            DeclarationName(self.0.into_into_static_str().into_static_str())
        }
    }
}

pub mod csr {
    use frender_common::{strings::CsrStr, IntoStaticStrCache, ToAsRefStr};

    use crate::csr::{CsrStyleStateUnmount, CssStyleDeclaration};

    use super::DeclarationName;

    pub trait UpdateStyleWithDeclarationName {
        fn update_style_with_declaration_name(self, name: DeclarationName<&str>);
        fn update_style_with_declaration_name_str(self, name: &str);
    }

    pub trait CsrDeclarationName {
        type Cacheable: IntoStaticStrCache<StaticStrCache = Self::StaticCache>;

        /// The [`CsrStyleStateUnmount`] allows to remove this style by name.
        type StaticCache: 'static + CsrStyleStateUnmount;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool;

        fn into_cacheable(this: Self) -> Self::Cacheable;

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationName);
    }

    pub struct DeclarationNameStrCacheable<S>(pub S);

    pub struct DeclarationNameStrStaticCache<S>(pub S);

    impl<S: IntoStaticStrCache> IntoStaticStrCache for DeclarationNameStrCacheable<S> {
        type StaticStrCache = DeclarationNameStrStaticCache<S::StaticStrCache>;

        fn into_static_str_cache(self) -> Self::StaticStrCache {
            DeclarationNameStrStaticCache(self.0.into_static_str_cache())
        }

        fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
            self.0.update_into_static_str_cache(&mut cache.0)
        }
    }

    impl<S: PartialEq<T>, T> PartialEq<DeclarationNameStrCacheable<T>>
        for DeclarationNameStrStaticCache<S>
    {
        fn eq(&self, other: &DeclarationNameStrCacheable<T>) -> bool {
            self.0 == other.0
        }
    }

    impl<S: ToAsRefStr> ToAsRefStr for DeclarationNameStrStaticCache<S> {
        type ToAsRefStr<'a> = S::ToAsRefStr<'a>
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self.0.to_as_ref_str()
        }
    }

    /// This relies on StrToAsRefStr::to_as_ref_str and StrToStaticCache::IntoToStaticCache would AsRef the same string
    impl<S: ToAsRefStr> CsrStyleStateUnmount for DeclarationNameStrStaticCache<S> {
        fn csr_style_state_unmount(this: &mut Self, style: &mut impl CssStyleDeclaration) {
            style.remove_property_str(this.0.to_as_ref_str().as_ref())
        }
    }

    impl<S: CsrStr> CsrDeclarationName for S {
        type Cacheable = DeclarationNameStrCacheable<S::IntoIntoStaticStrCache>;
        type StaticCache = DeclarationNameStrStaticCache<S::StaticStrCache>;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool {
            this.match_static_str_cache(&cache.0)
        }

        fn into_cacheable(this: Self) -> Self::Cacheable {
            DeclarationNameStrCacheable(this.into_into_static_str_cache())
        }

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationName) {
            style.update_style_with_declaration_name_str(this.to_as_ref_str().as_ref())
        }
    }

    pub struct CacheableDeclarationName<S>(S);
    pub struct CacheableDeclarationNameIntoStaticStrCache<S>(S);

    impl<S: ToAsRefStr> CsrStyleStateUnmount for CacheableDeclarationNameIntoStaticStrCache<S> {
        fn csr_style_state_unmount(this: &mut Self, style: &mut impl CssStyleDeclaration) {
            style.remove_property(DeclarationName(this.0.to_as_ref_str().as_ref()))
        }
    }

    impl<S: ToAsRefStr> ToAsRefStr for CacheableDeclarationName<S> {
        type ToAsRefStr<'a> = S::ToAsRefStr<'a>
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self.0.to_as_ref_str()
        }
    }

    impl<S: IntoStaticStrCache> IntoStaticStrCache for CacheableDeclarationName<S> {
        type StaticStrCache = CacheableDeclarationNameIntoStaticStrCache<S::StaticStrCache>;

        fn into_static_str_cache(self) -> Self::StaticStrCache {
            CacheableDeclarationNameIntoStaticStrCache(self.0.into_static_str_cache())
        }

        fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
            self.0.update_into_static_str_cache(&mut cache.0)
        }
    }

    impl<S: ToAsRefStr> ToAsRefStr for CacheableDeclarationNameIntoStaticStrCache<S> {
        type ToAsRefStr<'a> = S::ToAsRefStr<'a>
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self.0.to_as_ref_str()
        }
    }

    impl<S: PartialEq<T>, T> PartialEq<CacheableDeclarationName<T>>
        for CacheableDeclarationNameIntoStaticStrCache<S>
    {
        fn eq(&self, other: &CacheableDeclarationName<T>) -> bool {
            S::eq(&self.0, &other.0)
        }

        fn ne(&self, other: &CacheableDeclarationName<T>) -> bool {
            S::ne(&self.0, &other.0)
        }
    }

    /// This assumes StrToAsRefStr and StrToStaticCache are implemented in the way that the string value doesn't change
    impl<S: CsrStr> CsrDeclarationName for DeclarationName<S> {
        type Cacheable = CacheableDeclarationName<S::IntoIntoStaticStrCache>;
        type StaticCache = CacheableDeclarationNameIntoStaticStrCache<S::StaticStrCache>;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool {
            this.0.match_static_str_cache(&cache.0)
        }

        fn into_cacheable(this: Self) -> Self::Cacheable {
            CacheableDeclarationName(this.0.into_into_static_str_cache())
        }

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationName) {
            style.update_style_with_declaration_name(DeclarationName(
                this.0.to_as_ref_str().as_ref(),
            ))
        }
    }
}
