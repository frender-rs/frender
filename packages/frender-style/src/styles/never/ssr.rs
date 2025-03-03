use crate::ssr::{SsrDeclarationList, SsrStyle};

use super::Never;

impl SsrDeclarationList for Never {
    type IntoDeclarationList = async_str_iter::never::Never;

    type IntoDeclarationListPrefixSemicolon = async_str_iter::never::Never;

    fn into_declaration_list(this: Self) -> Self::IntoDeclarationList {
        match this {}
    }

    fn into_declaration_list_prefix_semicolon(
        this: Self,
    ) -> Self::IntoDeclarationListPrefixSemicolon {
        match this {}
    }
}

impl SsrStyle for Never {
    type IntoSsrDeclarationList = Self;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        this
    }
}
