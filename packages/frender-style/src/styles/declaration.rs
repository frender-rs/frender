//! One declaration as style

use crate::{declaration::IntoDeclaration, IntoStyle, Style};

pub struct IntoDeclarationAsStyle<D: IntoDeclaration>(pub D);

impl<D: IntoDeclaration> Style for IntoDeclarationAsStyle<D> {}

impl<D: IntoDeclaration> IntoStyle for D {
    type IntoStyle = IntoDeclarationAsStyle<D>;

    fn into_style(self) -> Self::IntoStyle {
        IntoDeclarationAsStyle(self)
    }
}

#[cfg(feature = "csr")]
pub(crate) mod csr;
#[cfg(feature = "ssr")]
pub(crate) mod ssr;
