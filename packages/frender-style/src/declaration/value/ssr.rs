use std::borrow::Borrow;

use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

use super::DeclarationValue;

pub trait IntoSsrDeclarationValue {
    // for ssr
    type StaticDeclarationValueStr: 'static + Borrow<str>;

    fn into_static_declaration_value(self) -> DeclarationValue<Self::StaticDeclarationValueStr>;
}

impl<S: StaticOrIntoStaticStr> IntoSsrDeclarationValue for S {
    type StaticDeclarationValueStr = S::StaticStr;

    fn into_static_declaration_value(self) -> DeclarationValue<Self::StaticDeclarationValueStr> {
        DeclarationValue::new(self.static_or_into_static_str())
    }
}

impl<S: StaticOrIntoStaticStr> IntoSsrDeclarationValue for DeclarationValue<S> {
    type StaticDeclarationValueStr = S::StaticStr;

    fn into_static_declaration_value(self) -> DeclarationValue<Self::StaticDeclarationValueStr> {
        // This assumes StrToAsRefStr and ToStaticStr are implemented in the way that the string value doesn't change
        DeclarationValue {
            unparsed: self.unparsed.static_or_into_static_str(),
        }
    }
}
