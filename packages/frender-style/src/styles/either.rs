pub enum EitherStyle<A, B> {
    A(A),
    B(B),
}

pub mod ssr {
    use crate::ssr::SsrStyle;

    use super::EitherStyle;

    use crate::ssr::SsrDeclarationList;

    pub enum EitherSsrDeclarationList<A: SsrDeclarationList, B: SsrDeclarationList> {
        A(A),
        B(B),
    }

    impl<A: SsrStyle, B: SsrStyle> SsrStyle for EitherStyle<A, B> {
        type IntoSsrDeclarationList =
            EitherSsrDeclarationList<A::IntoSsrDeclarationList, B::IntoSsrDeclarationList>;

        fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
            match this {
                EitherStyle::A(this) => {
                    EitherSsrDeclarationList::A(A::into_ssr_declaration_list(this))
                }
                EitherStyle::B(this) => {
                    EitherSsrDeclarationList::B(B::into_ssr_declaration_list(this))
                }
            }
        }
    }
}

#[cfg(feature = "either")]
mod extern_either {
    use either::Either;

    use crate::IntoStyle;

    use super::EitherStyle;

    impl<L, R> IntoStyle for Either<L, R> {
        type IntoStyle = EitherStyle<L, R>;

        fn into_style(self) -> Self::IntoStyle {
            match self {
                Either::Left(this) => EitherStyle::A(this),
                Either::Right(this) => EitherStyle::B(this),
            }
        }
    }
}
