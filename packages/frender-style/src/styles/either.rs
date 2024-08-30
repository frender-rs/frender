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

mod csr {
    use frender_common::either::EitherState;

    use crate::csr::CsrStyle;

    use super::EitherStyle;

    impl<L: CsrStyle, R: CsrStyle> CsrStyle for EitherStyle<L, R> {
        type UpdateWithState = EitherState<L::UpdateWithState, R::UpdateWithState>;

        fn update_with_state(
            this: Self,
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            match this {
                EitherStyle::A(this) => {
                    let state = match state {
                        EitherState::Left { inner: state } => state,
                        EitherState::Right { inner: old_state } => {
                            R::remove_with_state(old_state, style);
                            state.get_left_or_insert_default()
                        }
                    };

                    L::update_with_state(this, state, style)
                }
                EitherStyle::B(this) => {
                    let state = match state {
                        EitherState::Right { inner: state } => state,
                        EitherState::Left { inner: old_state } => {
                            L::remove_with_state(old_state, style);
                            state.get_right_or_insert_default()
                        }
                    };

                    R::update_with_state(this, state, style)
                }
            }
        }

        fn remove_with_state(
            state: &mut Self::UpdateWithState,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            match state {
                EitherState::Left { inner: state } => L::remove_with_state(state, style),
                EitherState::Right { inner: state } => R::remove_with_state(state, style),
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
