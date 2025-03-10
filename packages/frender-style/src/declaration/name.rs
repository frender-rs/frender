use std::borrow::Borrow;

use ccss::token::tokens::IdentToken;

use crate::styles::constness::const_value::{ConstBorrowStr, ConstValue, HasConstValue};

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

impl<'a, T: ?Sized + HasConstValue<Value: ConstBorrowStr<'a>>> DeclarationName<ConstValue<T>> {
    pub const fn new_const_value() -> Self {
        const {
            _ = DeclarationName::<&str>::new_const(
                <<T::Value as ConstBorrowStr<'a>>::HasConstValueStr<T> as HasConstValue>::VALUE,
            );
        }
        Self(ConstValue())
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
    ///
    /// This requires `N: Borrow<str>` to output the same string value,
    /// so this api currently is not exported.
    pub(crate) fn new(name: N) -> Self
    where
        N: Borrow<str>,
    {
        DeclarationName::<&str>::new_const(name.borrow());
        Self(name)
    }

    /// This relies on safe AsRef implementation
    pub(crate) fn as_borrowed(&self) -> DeclarationName<&str>
    where
        N: Borrow<str>,
    {
        DeclarationName(self.0.borrow())
    }
}

pub(crate) mod csr;
pub(crate) mod ssr;

mod sealed {
    pub trait IntoDeclarationName {}
}

pub trait IntoDeclarationName:
    sealed::IntoDeclarationName + ssr::IntoSsrDeclarationName + csr::IntoCsrDeclarationName
{
}

mod imps {
    use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

    use super::{csr::CsrStr, sealed, DeclarationName, IntoDeclarationName};

    impl<S: StaticOrIntoStaticStr + CsrStr> sealed::IntoDeclarationName for S {}
    impl<S: StaticOrIntoStaticStr + CsrStr> IntoDeclarationName for S {}

    impl<N: StaticOrIntoStaticStr + CsrStr> sealed::IntoDeclarationName for DeclarationName<N> {}
    impl<N: StaticOrIntoStaticStr + CsrStr> IntoDeclarationName for DeclarationName<N> {}
}
