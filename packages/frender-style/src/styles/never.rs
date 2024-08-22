pub enum Never {}

impl Never {
    pub fn assert(this: Self) -> Self {
        this
    }
}

mod ssr {
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
}

mod csr {
    use crate::csr::CsrStyle;

    use super::Never;

    impl CsrStyle for Never {
        type UpdateWithState = ();

        fn update_with_state(
            this: Self,
            _: &mut Self::UpdateWithState,
            _: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            match this {}
        }

        fn remove_with_state(
            _: &mut Self::UpdateWithState,
            _: &mut impl crate::csr::CssStyleDeclaration,
        ) {
        }
    }
}

#[cfg(test)]
#[test]
fn as_never() {
    fn cast(never: std::convert::Infallible) -> Never {
        (|| -> Never { match never {} })()
    }

    None::<std::convert::Infallible>.map(cast);
}
