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
    use crate::csr::{CsrStyle, CsrStyleStateUnmount};

    use super::EitherStyle;

    pub enum State<A, B> {
        A(A),
        B(B),
    }

    impl<A: CsrStyleStateUnmount, B: CsrStyleStateUnmount> CsrStyleStateUnmount for State<A, B> {
        fn csr_style_state_unmount(
            state: &mut Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            match state {
                State::A(state) => A::csr_style_state_unmount(state, style),
                State::B(state) => B::csr_style_state_unmount(state, style),
            }
        }
    }

    impl<L: CsrStyle, R: CsrStyle> CsrStyle for EitherStyle<L, R> {
        type State = State<L::State, R::State>;

        fn csr_style_render_init(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) -> Self::State {
            match this {
                EitherStyle::A(this) => State::A(L::csr_style_render_init(this, style)),
                EitherStyle::B(this) => State::B(R::csr_style_render_init(this, style)),
            }
        }

        fn csr_style_render_init_with_old_state(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
            old_state: &mut Self::State,
        ) {
            match this {
                EitherStyle::A(this) => match old_state {
                    State::A(old_state) => {
                        L::csr_style_render_init_with_old_state(this, style, old_state)
                    }
                    State::B(_) => {
                        // already unmounted
                        *old_state = State::A(L::csr_style_render_init(this, style))
                    }
                },
                EitherStyle::B(this) => match old_state {
                    State::B(old_state) => {
                        R::csr_style_render_init_with_old_state(this, style, old_state)
                    }
                    State::A(_) => {
                        // already unmounted
                        *old_state = State::B(R::csr_style_render_init(this, style))
                    }
                },
            }
        }

        fn csr_style_render_update(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
            state: &mut Self::State,
        ) {
            match this {
                EitherStyle::A(this) => match state {
                    State::A(state) => L::csr_style_render_update(this, style, state),
                    State::B(old_state) => {
                        <R::State>::csr_style_state_unmount(old_state, style);

                        *state = State::A(L::csr_style_render_init(this, style))
                    }
                },
                EitherStyle::B(this) => {
                    match state {
                        State::B(state) => R::csr_style_render_update(this, style, state),
                        State::A(old_state) => {
                            <L::State>::csr_style_state_unmount(old_state, style);
                            *state = State::B(R::csr_style_render_init(this, style))
                        }
                    };
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
