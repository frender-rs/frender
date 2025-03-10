use crate::{css_style_declaration::CssStyleDeclaration, IntoStyle};

pub trait CsrStyleStateUnmount {
    /// Takes `&mut Self` instead of `Self` so that:
    /// - [`EitherStyle`](crate::styles::EitherStyle) can ensure old state is unmounted before new state is initialized,
    ///   avoiding the case where they share same style declaration names.
    /// - `Preserved` can be implemented without additional traits.
    fn csr_style_state_unmount(state: &mut Self, style: &mut impl CssStyleDeclaration);
}

impl CsrStyleStateUnmount for () {
    fn csr_style_state_unmount((): &mut Self, _: &mut impl CssStyleDeclaration) {}
}

pub trait CsrStyle {
    type State: CsrStyleStateUnmount;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State;

    /// The `old_state` was [unmounted](CsrStyleStateUnmount::csr_style_state_unmount).
    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        old_state: &mut Self::State,
    ) where
        Self: Sized,
    {
        *old_state = Self::csr_style_render_init(this, style)
    }

    fn csr_style_render_update(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        state: &mut Self::State,
    );
}

impl<S: IntoStyle> CsrStyle for S {
    type State = <S::IntoStyle as CsrStyle>::State;

    fn csr_style_render_init(this: Self, style: &mut impl CssStyleDeclaration) -> Self::State {
        <S::IntoStyle>::csr_style_render_init(this.into_style(), style)
    }

    fn csr_style_render_init_with_old_state(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        old_state: &mut Self::State,
    ) {
        <S::IntoStyle>::csr_style_render_init_with_old_state(this.into_style(), style, old_state)
    }

    fn csr_style_render_update(
        this: Self,
        style: &mut impl CssStyleDeclaration,
        state: &mut Self::State,
    ) {
        <S::IntoStyle>::csr_style_render_update(this.into_style(), style, state)
    }
}
