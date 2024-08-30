use ccss::collections::{
    component_value_list::{IsKnownComponentValueList, KnownComponentValueList},
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
    use frender_common::{strings::StrToStaticStr, ToStaticStr};

    use super::DeclarationValue;

    pub trait IntoSsrDeclarationValue {
        // for ssr
        type StaticDeclarationValueStr: 'static + AsRef<str>;

        fn into_static_declaration_value(self)
            -> DeclarationValue<Self::StaticDeclarationValueStr>;
    }

    impl<S: StrToStaticStr> IntoSsrDeclarationValue for S {
        type StaticDeclarationValueStr = S::StaticStr;

        fn into_static_declaration_value(
            self,
        ) -> DeclarationValue<Self::StaticDeclarationValueStr> {
            DeclarationValue::new(self.into_to_static_str().into_static_str())
        }
    }

    impl<S: StrToStaticStr> IntoSsrDeclarationValue for DeclarationValue<S> {
        type StaticDeclarationValueStr = S::StaticStr;

        fn into_static_declaration_value(
            self,
        ) -> DeclarationValue<Self::StaticDeclarationValueStr> {
            // This assumes StrToAsRefStr and ToStaticStr are implemented in the way that the string value doesn't change
            DeclarationValue {
                unparsed: self.unparsed.into_to_static_str().into_static_str(),
            }
        }
    }
}

pub mod csr {
    use frender_common::{
        strings::{StrToAsRefStr, StrToStaticCache, StrToStaticStr},
        ToAsRefStr, ToStaticCache,
    };

    use super::DeclarationValue;

    pub trait UpdateStyleWithDeclarationValue {
        fn update_style_with_declaration_value(self, value: DeclarationValue<&str>);
        fn update_style_with_declaration_value_str(self, value: &str);
    }

    pub trait CsrDeclarationValue {
        type Cacheable: ToStaticCache<StaticCache = Self::StaticCache>;
        type StaticCache: 'static;

        fn into_cacheable(this: Self) -> Self::Cacheable;

        fn update_style(this: &Self::Cacheable, style: impl UpdateStyleWithDeclarationValue);
    }

    impl<S: StrToStaticStr + StrToStaticCache> CsrDeclarationValue for S {
        type Cacheable = S::IntoToStaticCache;
        type StaticCache = S::StaticCache;

        fn into_cacheable(this: Self) -> Self::Cacheable {
            this.into_to_static_cache()
        }

        fn update_style(this: &Self::Cacheable, style: impl UpdateStyleWithDeclarationValue) {
            style.update_style_with_declaration_value_str(this.to_as_ref_str().as_ref())
        }
    }

    pub struct CacheableDeclarationValue<S: ToStaticCache>(S);

    impl<S: ToStaticCache> ToStaticCache for CacheableDeclarationValue<S> {
        type StaticCache = S::StaticCache;
        frender_common::proxy_to_static_cache!(|self| -> S { self.0 });
    }

    impl<S: StrToStaticStr + StrToStaticCache> CsrDeclarationValue for DeclarationValue<S> {
        type Cacheable = CacheableDeclarationValue<S::IntoToStaticCache>;
        type StaticCache = S::StaticCache;

        fn into_cacheable(this: Self) -> Self::Cacheable {
            CacheableDeclarationValue(this.into_unparsed().into_to_static_cache())
        }

        fn update_style(this: &Self::Cacheable, style: impl UpdateStyleWithDeclarationValue) {
            style.update_style_with_declaration_value(DeclarationValue {
                unparsed: this.0.to_as_ref_str().as_ref(),
            })
        }
    }
}
