use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount},
    css_style_declaration::CssStyleDeclaration,
};

use super::Chain;

impl<A: CsrStyleStateUnmount, B: CsrStyleStateUnmount> CsrStyleStateUnmount for (A, B) {
    fn csr_style_state_unmount(
        (state_a, state_b): &mut Self,
        style: &mut impl CssStyleDeclaration,
    ) {
        A::csr_style_state_unmount(state_a, style);
        B::csr_style_state_unmount(state_b, style);
    }
}

impl<A: CsrStyle, B: CsrStyle> CsrStyle for Chain<A, B> {
    type State = (A::State, B::State);

    fn csr_style_render_init(
        Self(a, b): Self,
        style: &mut impl CssStyleDeclaration,
    ) -> Self::State {
        (
            A::csr_style_render_init(a, style),
            B::csr_style_render_init(b, style),
        )
    }

    fn csr_style_render_init_with_old_state(
        Self(a, b): Self,
        style: &mut impl CssStyleDeclaration,
        (old_state_a, old_state_b): &mut Self::State,
    ) {
        A::csr_style_render_init_with_old_state(a, style, old_state_a);
        B::csr_style_render_init_with_old_state(b, style, old_state_b);
    }

    fn csr_style_render_update(
        Self(a, b): Self,
        style: &mut impl CssStyleDeclaration,
        (state_a, state_b): &mut Self::State,
    ) {
        A::csr_style_render_update(a, style, state_a);
        B::csr_style_render_update(b, style, state_b);
    }
}
