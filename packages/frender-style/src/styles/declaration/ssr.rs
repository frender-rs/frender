use async_str_iter::{any_str::IterAnyStr, AsyncStrIterator};

use crate::{
    declaration::{
        important::ssr::{assert::BangImportantOrEmpty, IntoSsrDeclarationImportant},
        name::ssr::IntoSsrDeclarationName,
        value::ssr::IntoSsrDeclarationValue,
        Declaration, DeclarationName, DeclarationValue, IntoDeclaration, IntoDeclarationAsStyle,
    },
    ssr::{SsrDeclarationList, SsrStyle},
};

impl<D: IntoDeclaration> SsrStyle for IntoDeclarationAsStyle<D> {
    type IntoSsrDeclarationList = Self;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        this
    }
}

impl<D: IntoDeclaration> SsrDeclarationList for IntoDeclarationAsStyle<D> {
    type IntoDeclarationList = OneDeclarationAsList<
        IterAnyStr<<D::Name as IntoSsrDeclarationName>::StaticDeclarationNameStr>,
        IterAnyStr<<D::Value as IntoSsrDeclarationValue>::StaticDeclarationValueStr>,
        <D::Important as IntoSsrDeclarationImportant>::BangImportant,
    >;

    type IntoDeclarationListPrefixSemicolon = OneDeclarationAsListPrefixSemicolon<
        IterAnyStr<<D::Name as IntoSsrDeclarationName>::StaticDeclarationNameStr>,
        IterAnyStr<<D::Value as IntoSsrDeclarationValue>::StaticDeclarationValueStr>,
        <D::Important as IntoSsrDeclarationImportant>::BangImportant,
    >;

    fn into_declaration_list(this: Self) -> Self::IntoDeclarationList {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        OneDeclarationAsList::new(
            name.into_static_declaration_name(),
            value.into_static_declaration_value(),
            important.into_ssr_declaration_important(),
        )
    }

    fn into_declaration_list_prefix_semicolon(
        this: Self,
    ) -> Self::IntoDeclarationListPrefixSemicolon {
        let Declaration {
            name,
            value,
            important,
        } = this.0.into_declaration();

        OneDeclarationAsListPrefixSemicolon::new(
            name.into_static_declaration_name(),
            value.into_static_declaration_value(),
            important.into_ssr_declaration_important(),
        )
    }
}

async_str_iter::Strings!(
    enum OneDeclarationAsListState {}
    /// This can only be constructed from valid [`DeclarationName`] and [`DeclarationValue`].
    pub struct OneDeclarationAsList<N: AsyncStrIterator, V: AsyncStrIterator, I: BangImportantOrEmpty>(
        name!(N),
        colon!(":"),
        value!(V),
        bang_important_or_empty!(I),
    );
);

impl<N: AsRef<str>, V: AsRef<str>, I: BangImportantOrEmpty>
    OneDeclarationAsList<IterAnyStr<N>, IterAnyStr<V>, I>
{
    pub fn new(
        name: DeclarationName<N>,
        value: DeclarationValue<V>,
        bang_important_or_empty: I,
    ) -> Self {
        Self {
            _state: OneDeclarationAsListState(),
            name: IterAnyStr::new(name.into_unparsed()),
            colon: (),
            value: IterAnyStr::new(value.into_unparsed()),
            bang_important_or_empty,
        }
    }
}

async_str_iter::Strings!(
    enum OneDeclarationAsListPrefixSemicolonState {}
    /// This can only be constructed from valid [`DeclarationName`] and [`DeclarationValue`].
    pub struct OneDeclarationAsListPrefixSemicolon<
        N: AsyncStrIterator,
        V: AsyncStrIterator,
        I: BangImportantOrEmpty,
    >(
        semicolon!(";"),
        name!(N),
        colon!(":"),
        value!(V),
        bang_important_or_empty!(I),
    );
);

impl<N: AsRef<str>, V: AsRef<str>, I: BangImportantOrEmpty>
    OneDeclarationAsListPrefixSemicolon<IterAnyStr<N>, IterAnyStr<V>, I>
{
    pub fn new(
        name: DeclarationName<N>,
        value: DeclarationValue<V>,
        bang_important_or_empty: I,
    ) -> Self {
        Self {
            _state: OneDeclarationAsListPrefixSemicolonState(),
            semicolon: (),
            name: IterAnyStr::new(name.into_unparsed()),
            colon: (),
            value: IterAnyStr::new(value.into_unparsed()),
            bang_important_or_empty,
        }
    }
}
