mod csr {
    use frender_common::Empty;

    use crate::csr::CsrStyle;

    impl CsrStyle for Empty {
        type State = ();

        fn csr_style_render_init(
            Self: Self,
            _: &mut impl crate::csr::CssStyleDeclaration,
        ) -> Self::State {
        }

        fn csr_style_render_update(
            Self: Self,
            _: &mut impl crate::csr::CssStyleDeclaration,
            (): &mut Self::State,
        ) {
        }
    }
}

mod ssr {
    use frender_common::Empty;

    use crate::ssr::SsrStyle;

    impl SsrStyle for Empty {
        type IntoSsrDeclarationList = Self;

        fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
            this
        }
    }
}
