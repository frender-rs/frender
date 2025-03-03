use crate::ssr::SsrStyle;

use super::Chain;

impl<A: SsrStyle, B: SsrStyle> SsrStyle for Chain<A, B> {
    type IntoSsrDeclarationList = Chain<A::IntoSsrDeclarationList, B::IntoSsrDeclarationList>;

    fn into_ssr_declaration_list(Self(a, b): Self) -> Self::IntoSsrDeclarationList {
        Chain(
            A::into_ssr_declaration_list(a),
            B::into_ssr_declaration_list(b),
        )
    }
}
