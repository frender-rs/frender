use frender_common::Empty;

use crate::ssr::SsrStyle;

impl SsrStyle for Empty {
    type IntoSsrDeclarationList = Self;

    fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
        this
    }
}
