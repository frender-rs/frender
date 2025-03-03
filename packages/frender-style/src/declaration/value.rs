use std::borrow::Borrow;

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
    ///
    /// Not public api due to the reason described in [`crate::declaration::DeclarationName::new()`]
    pub(crate) fn new(unparsed: S) -> Self
    where
        S: Borrow<str>,
    {
        _ = DeclarationValue::<&str>::new_const(unparsed.borrow());
        Self { unparsed }
    }

    pub const fn unparsed(&self) -> &S {
        &self.unparsed
    }

    pub fn into_unparsed(self) -> S {
        self.unparsed
    }

    /// This relies on safe AsRef implementation
    pub(crate) fn as_borrowed(&self) -> DeclarationValue<&str>
    where
        S: Borrow<str>,
    {
        DeclarationValue {
            unparsed: self.unparsed.borrow(),
        }
    }
}

pub mod csr;
pub mod ssr;

pub trait IntoDeclarationValue:
    ssr::IntoSsrDeclarationValue + csr::IntoCsrDeclarationValue
{
}
impl<T: ssr::IntoSsrDeclarationValue + csr::IntoCsrDeclarationValue> IntoDeclarationValue for T {}
