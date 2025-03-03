use frender_common::Empty;

use crate::{csr::CsrStyle, css_style_declaration::CssStyleDeclaration};

impl CsrStyle for Empty {
    type State = ();

    fn csr_style_render_init(Self: Self, _: &mut impl CssStyleDeclaration) -> Self::State {}

    fn csr_style_render_update(Self: Self, _: &mut impl CssStyleDeclaration, (): &mut Self::State) {
    }
}
