use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount},
    css_style_declaration::CssStyleDeclaration,
};

use super::EitherStyle;

pub enum State<A, B> {
    A(A),
    B(B),
}

impl<A: CsrStyleStateUnmount, B: CsrStyleStateUnmount> CsrStyleStateUnmount for State<A, B> {
    fn csr_style_state_unmount(state: &mut Self, style: &mut impl CssStyleDeclaration) {
        match state {
            State::A(state) => A::csr_style_state_unmount(state, style),
            State::B(state) => B::csr_style_state_unmount(state, style),
        }
    }
}

impl<L: CsrStyle, R: CsrStyle> CsrStyle for EitherStyle<L, R> {
    type State = State<L::State, R::State>;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        match this {
            EitherStyle::A(this) => State::A(L::csr_style_render_init(this, style)),
            EitherStyle::B(this) => State::B(R::csr_style_render_init(this, style)),
        }
    }

    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
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
        style: &mut impl CssStyleDeclaration,
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
