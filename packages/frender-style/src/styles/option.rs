mod ssr {
    use crate::ssr::SsrStyle;

    impl<T: SsrStyle> SsrStyle for Option<T> {
        type IntoSsrDeclarationList = Option<T::IntoSsrDeclarationList>;

        fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
            this.map(T::into_ssr_declaration_list)
        }
    }
}

mod csr {
    use crate::csr::CsrStyle;

    impl<T: CsrStyle> CsrStyle for Option<T> {
        type UpdateWithState = T::UpdateWithState;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            match this {
                Some(this) => T::update_with_state(this, state, style),
                None => T::remove_with_state(state, style),
            }
        }

        fn remove_with_state(
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            T::remove_with_state(state, style)
        }
    }
}
