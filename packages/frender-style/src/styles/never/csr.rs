use crate::{
    csr::{CsrStyle, CsrStyleStateUnmount},
    css_style_declaration::CssStyleDeclaration,
};

use super::Never;

impl CsrStyleStateUnmount for Never {
    fn csr_style_state_unmount(state: &mut Self, _: &mut impl CssStyleDeclaration) {
        match *state {}
    }
}

impl CsrStyle for Never {
    type State = Never;

    fn csr_style_render_init(this: Self, _: &mut impl CssStyleDeclaration) -> Self::State {
        match this {}
    }

    fn csr_style_render_init_with_old_state(
        this: Self,
        _: &mut impl CssStyleDeclaration,
        _: &mut Self::State,
    ) {
        match this {}
    }

    fn csr_style_render_update(this: Self, _: &mut impl CssStyleDeclaration, _: &mut Self::State) {
        match this {}
    }
}
