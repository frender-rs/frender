pub use self::{name::DeclarationName, value::DeclarationValue};

use important::IntoDeclarationImportant;
use name::IntoDeclarationName;
use value::IntoDeclarationValue;

pub mod important;
pub mod name;
pub mod value;

/// https://drafts.csswg.org/css-syntax-3/#parse-declaration
#[derive(Debug, Clone, Copy)]
pub struct Declaration<
    N: IntoDeclarationName,
    V: IntoDeclarationValue,
    I: IntoDeclarationImportant = frender_common::Empty,
> {
    pub name: N,
    pub value: V,
    pub important: I,
}

pub trait IntoDeclaration {
    type Name: IntoDeclarationName;
    type Value: IntoDeclarationValue;
    type Important: IntoDeclarationImportant;

    fn into_declaration(self) -> Declaration<Self::Name, Self::Value, Self::Important>;
}

impl<N: IntoDeclarationName, V: IntoDeclarationValue, I: IntoDeclarationImportant> IntoDeclaration
    for Declaration<N, V, I>
{
    type Name = N;
    type Value = V;
    type Important = I;

    fn into_declaration(self) -> Declaration<Self::Name, Self::Value, Self::Important> {
        self
    }
}

impl<N: IntoDeclarationName, V: IntoDeclarationValue> IntoDeclaration for (N, V) {
    type Name = N;
    type Value = V;
    type Important = frender_common::Empty;

    fn into_declaration(self) -> Declaration<Self::Name, Self::Value, Self::Important> {
        Declaration {
            name: self.0,
            value: self.1,
            important: frender_common::Empty,
        }
    }
}

impl<N: IntoDeclarationName, V: IntoDeclarationValue, I: IntoDeclarationImportant> IntoDeclaration
    for (N, V, I)
{
    type Name = N;
    type Value = V;
    type Important = I;

    fn into_declaration(self) -> Declaration<Self::Name, Self::Value, Self::Important> {
        Declaration {
            name: self.0,
            value: self.1,
            important: self.2,
        }
    }
}
