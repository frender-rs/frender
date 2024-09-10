use ccss::collections::{
    component_value_list::IsKnownComponentValueList,
    declaration_value_list::KnownDeclarationValueList,
};

/// A list of component values excluding important flag.
///
/// This might be empty.
///
/// https://drafts.csswg.org/css-syntax-3/#consume-list-of-components
#[derive(Debug, Clone, Copy)]
pub struct DeclarationValue<V> {
    unparsed: V,
}

const fn slices_equal(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }

    true
}

impl<'a> DeclarationValue<&'a str> {
    pub(crate) const fn new_const(s: &'a str) -> Self {
        let parsed = ccss::parse::declaration::Declaration::<
            ccss::collections::collect_nothing::CollectNothing,
        >::parse_value_from_str(s);

        assert!(
            slices_equal(parsed.full_as_str().as_bytes(), s.as_bytes()),
            "string got trimmed, it might contain leading or trailing whitespace"
        );
        Self::from_parsed(&parsed)
    }

    pub(crate) const fn from_parsed<L: IsKnownComponentValueList<'a>>(
        parsed: &KnownDeclarationValueList<'a, L>,
    ) -> Self {
        Self {
            unparsed: parsed.full_as_str(),
        }
    }
}

impl<S> DeclarationValue<S> {
    /// Panics if `name.as_ref()` is not valid declaration value list.
    pub fn new(unparsed: S) -> Self
    where
        S: AsRef<str>,
    {
        _ = DeclarationValue::<&str>::new_const(unparsed.as_ref());
        Self { unparsed }
    }

    pub const fn unparsed(&self) -> &S {
        &self.unparsed
    }

    pub fn into_unparsed(self) -> S {
        self.unparsed
    }

    /// This relies on safe AsRef implementation
    pub(crate) fn as_ref_str(&self) -> DeclarationValue<&str>
    where
        S: AsRef<str>,
    {
        DeclarationValue {
            unparsed: self.unparsed.as_ref(),
        }
    }
}

pub trait IntoDeclarationValue: ssr::IntoSsrDeclarationValue + csr::CsrDeclarationValue {}

impl<T: ssr::IntoSsrDeclarationValue + csr::CsrDeclarationValue> IntoDeclarationValue for T {}

pub mod ssr {
    use frender_common::{strings::SsrStr, IntoStaticStr};

    use super::DeclarationValue;

    pub trait IntoSsrDeclarationValue {
        // for ssr
        type StaticDeclarationValueStr: 'static + AsRef<str>;

        fn into_static_declaration_value(self)
            -> DeclarationValue<Self::StaticDeclarationValueStr>;
    }

    impl<S: SsrStr> IntoSsrDeclarationValue for S {
        type StaticDeclarationValueStr = S::StaticStr;

        fn into_static_declaration_value(
            self,
        ) -> DeclarationValue<Self::StaticDeclarationValueStr> {
            DeclarationValue::new(self.into_into_static_str().into_static_str())
        }
    }

    impl<S: SsrStr> IntoSsrDeclarationValue for DeclarationValue<S> {
        type StaticDeclarationValueStr = S::StaticStr;

        fn into_static_declaration_value(
            self,
        ) -> DeclarationValue<Self::StaticDeclarationValueStr> {
            // This assumes StrToAsRefStr and ToStaticStr are implemented in the way that the string value doesn't change
            DeclarationValue {
                unparsed: self.unparsed.into_into_static_str().into_static_str(),
            }
        }
    }
}

pub mod csr {

    use frender_common::{strings::CsrStr, IntoStaticStrCache, ToAsRefStr};

    use super::DeclarationValue;

    pub trait UpdateStyleWithDeclarationValue {
        fn update_style_with_declaration_value(self, value: DeclarationValue<&str>);
        fn update_style_with_declaration_value_str(self, value: &str);
    }

    pub trait CsrDeclarationValue {
        type Cacheable: IntoStaticStrCache<StaticStrCache = Self::StaticCache>;
        type StaticCache: 'static;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool;

        fn into_cacheable(this: Self) -> Self::Cacheable;

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationValue);
    }

    impl<S: CsrStr> CsrDeclarationValue for S {
        type Cacheable = S::IntoIntoStaticStrCache;
        type StaticCache = S::StaticStrCache;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool {
            cache == this
        }

        fn into_cacheable(this: Self) -> Self::Cacheable {
            this.into_into_static_str_cache()
        }

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationValue) {
            style.update_style_with_declaration_value_str(this.to_as_ref_str().as_ref())
        }
    }

    pub struct CacheableDeclarationValue<S>(S);

    pub struct CacheableDeclarationValueIntoStaticStrCache<S>(S);

    impl<S: ToAsRefStr> ToAsRefStr for CacheableDeclarationValue<S> {
        type ToAsRefStr<'a> = S::ToAsRefStr<'a>
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self.0.to_as_ref_str()
        }
    }

    impl<S: IntoStaticStrCache> IntoStaticStrCache for CacheableDeclarationValue<S> {
        type StaticStrCache = CacheableDeclarationValueIntoStaticStrCache<S::StaticStrCache>;

        fn into_static_str_cache(self) -> Self::StaticStrCache {
            CacheableDeclarationValueIntoStaticStrCache(self.0.into_static_str_cache())
        }

        fn update_into_static_str_cache(self, cache: &mut Self::StaticStrCache) {
            self.0.update_into_static_str_cache(&mut cache.0)
        }
    }

    impl<S: ToAsRefStr> ToAsRefStr for CacheableDeclarationValueIntoStaticStrCache<S> {
        type ToAsRefStr<'a> = S::ToAsRefStr<'a>
        where
            Self: 'a;

        fn to_as_ref_str(&self) -> Self::ToAsRefStr<'_> {
            self.0.to_as_ref_str()
        }
    }

    impl<S: PartialEq<T>, T> PartialEq<CacheableDeclarationValue<T>>
        for CacheableDeclarationValueIntoStaticStrCache<S>
    {
        fn eq(&self, other: &CacheableDeclarationValue<T>) -> bool {
            S::eq(&self.0, &other.0)
        }

        fn ne(&self, other: &CacheableDeclarationValue<T>) -> bool {
            S::ne(&self.0, &other.0)
        }
    }

    impl<S: CsrStr> CsrDeclarationValue for DeclarationValue<S> {
        type Cacheable = CacheableDeclarationValue<S::IntoIntoStaticStrCache>;
        type StaticCache = CacheableDeclarationValueIntoStaticStrCache<S::StaticStrCache>;

        fn match_cache(this: &Self, cache: &Self::StaticCache) -> bool {
            cache.0 == this.unparsed
        }

        fn into_cacheable(this: Self) -> Self::Cacheable {
            CacheableDeclarationValue(this.into_unparsed().into_into_static_str_cache())
        }

        fn update_style(this: &Self::StaticCache, style: impl UpdateStyleWithDeclarationValue) {
            style.update_style_with_declaration_value(DeclarationValue {
                unparsed: this.0.to_as_ref_str().as_ref(),
            })
        }
    }
}
