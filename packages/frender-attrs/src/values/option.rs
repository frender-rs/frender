pub struct OptionAttributes<T>(pub(super) Option<T>);

use crate::Attributes;

impl<T: Attributes> crate::sealed::Attributes for OptionAttributes<T> {}
impl<T: Attributes> Attributes for OptionAttributes<T> {}

#[cfg(feature = "csr")]
mod csr {
    use crate::csr::{CsrAttributes, CsrAttributesStateUnmount};

    use super::OptionAttributes;

    pub struct OptionState<S>(Option<S>);

    impl<S: CsrAttributesStateUnmount> CsrAttributesStateUnmount for OptionState<S> {
        fn state_unmount(&mut self, renderer: &mut impl crate::csr::RenderAttributes) {
            if let Some(ref mut state) = self.0.take() {
                S::state_unmount(state, renderer);
            }
        }
    }

    impl<T: CsrAttributes> CsrAttributes for OptionAttributes<T> {
        type State = OptionState<T::State>;

        fn render_init(self, renderer: &mut impl crate::csr::RenderAttributes) -> Self::State {
            OptionState(if let Some(this) = self.0 {
                Some(T::render_init(this, renderer))
            } else {
                None
            })
        }

        fn render_update(
            self,
            renderer: &mut impl crate::csr::RenderAttributes,
            state: &mut Self::State,
        ) {
            match self.0 {
                Some(this) => match &mut state.0 {
                    Some(state) => this.render_update(renderer, state),
                    state @ None => *state = Some(this.render_init(renderer)),
                },
                None => state.state_unmount(renderer),
            }
        }
    }
}
#[cfg(feature = "ssr")]
mod ssr {
    use async_str_iter::{option::IterOption, IntoAsyncStrIterator};

    use crate::ssr::SsrAttributes;

    use super::OptionAttributes;

    impl<T: SsrAttributes> SsrAttributes for OptionAttributes<T> {
        type IntoSsrAttributes = IterOption<T::IntoSsrAttributes>;

        fn into_ssr_attributes(self) -> Self::IntoSsrAttributes {
            self.0.map(T::into_ssr_attributes).into_async_str_iterator()
        }
    }
}
