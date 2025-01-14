mod ssr {
    use crate::ssr::SsrStyle;

    impl<T: SsrStyle> SsrStyle for Option<T> {
        type IntoSsrDeclarationList = Option<T::IntoSsrDeclarationList>;

        fn into_ssr_declaration_list(this: Self) -> Self::IntoSsrDeclarationList {
            this.map(T::into_ssr_declaration_list)
        }
    }
}

mod csr {
    use crate::csr::{CsrStyle, CsrStyleStateUnmount};

    impl<T: CsrStyleStateUnmount> CsrStyleStateUnmount for Option<T> {
        fn csr_style_state_unmount(
            state: &mut Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) {
            if let Some(state) = state {
                T::csr_style_state_unmount(state, style);
            }

            *state = None; // drop the state
        }
    }

    impl<T: CsrStyle> CsrStyle for Option<T> {
        type State = Option<T::State>;

        fn csr_style_render_init(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
        ) -> Self::State {
            if let Some(this) = this {
                Some(T::csr_style_render_init(this, style))
            } else {
                None
            }
        }

        fn csr_style_render_init_with_old_state(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
            old_state: &mut Self::State,
        ) {
            if let Some(this) = this {
                if let Some(old_state) = old_state {
                    T::csr_style_render_init_with_old_state(this, style, old_state);
                } else {
                    *old_state = Some(T::csr_style_render_init(this, style))
                }
            } else {
                // old_state had been unmounted, so just drop it
                *old_state = None;
            }
        }

        fn csr_style_render_update(
            this: Self,
            style: &mut impl crate::csr::CssStyleDeclaration,
            state: &mut Self::State,
        ) {
            if let Some(this) = this {
                if let Some(state) = state {
                    T::csr_style_render_update(this, style, state)
                } else {
                    *state = Some(T::csr_style_render_init(this, style))
                }
            } else {
                Self::State::csr_style_state_unmount(state, style)
            }
        }
    }
}
