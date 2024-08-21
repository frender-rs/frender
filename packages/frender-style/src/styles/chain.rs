#[derive(Debug, Clone, Copy)]
pub struct Chain<A, B>(pub A, pub B);

mod ssr {
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
}

mod csr {
    use crate::csr::CsrStyle;

    use super::Chain;

    impl<A: CsrStyle, B: CsrStyle> CsrStyle for Chain<A, B> {
        type UpdateWithState = (A::UpdateWithState, B::UpdateWithState);

        fn update_with_state(
            Self(a, b): Self,
            (state_a, state_b): &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            A::update_with_state(a, state_a, style);
            B::update_with_state(b, state_b, style);
        }

        fn remove_with_state(
            (state_a, state_b): &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            A::remove_with_state(state_a, style);
            B::remove_with_state(state_b, style);
        }
    }
}
