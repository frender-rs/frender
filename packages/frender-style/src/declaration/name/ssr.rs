use std::borrow::Borrow;

use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

use super::DeclarationName;

pub trait IntoSsrDeclarationName {
    type StaticDeclarationNameStr: 'static + Borrow<str>;

    fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr>;
}

impl<S: StaticOrIntoStaticStr> IntoSsrDeclarationName for S {
    type StaticDeclarationNameStr = S::StaticStr;

    fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr> {
        DeclarationName::new(self.static_or_into_static_str())
    }
}

impl<N: StaticOrIntoStaticStr> IntoSsrDeclarationName for DeclarationName<N> {
    type StaticDeclarationNameStr = N::StaticStr;

    fn into_static_declaration_name(self) -> DeclarationName<Self::StaticDeclarationNameStr> {
        // This assumes StrToAsRefStr and ToStaticStr are implemented in the way that the string value doesn't change
        DeclarationName(self.0.static_or_into_static_str())
    }
}
