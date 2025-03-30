use crate::Attributes;

use super::EitherAttributes;

impl<A: Attributes, B: Attributes> crate::sealed::Attributes for EitherAttributes<A, B> {}
impl<A: Attributes, B: Attributes> Attributes for EitherAttributes<A, B> {}

#[cfg(feature = "csr")]
mod csr {
    use crate::{
        csr::{CsrAttributes, CsrAttributesStateUnmount},
        values::EitherAttributes,
    };

    pub enum EitherState<A, B> {
        A(A),
        B(B),
    }

    impl<A: CsrAttributesStateUnmount, B: CsrAttributesStateUnmount> CsrAttributesStateUnmount
        for EitherState<A, B>
    {
        fn state_unmount(&mut self, renderer: &mut impl crate::csr::RenderAttributes) {
            match self {
                EitherState::A(state) => state.state_unmount(renderer),
                EitherState::B(state) => state.state_unmount(renderer),
            }
        }
    }

    impl<A: CsrAttributes, B: CsrAttributes> CsrAttributes for EitherAttributes<A, B> {
        type State = EitherState<A::State, B::State>;

        fn render_init(self, renderer: &mut impl crate::csr::RenderAttributes) -> Self::State {
            match self {
                EitherAttributes::A(this) => EitherState::A(this.render_init(renderer)),
                EitherAttributes::B(this) => EitherState::B(this.render_init(renderer)),
            }
        }

        fn render_update(
            self,
            renderer: &mut impl crate::csr::RenderAttributes,
            state: &mut Self::State,
        ) {
            match self {
                EitherAttributes::A(this) => match state {
                    EitherState::A(state) => A::render_update(this, renderer, state),
                    EitherState::B(old_state) => {
                        old_state.state_unmount(renderer);
                        *state = EitherState::A(this.render_init(renderer));
                    }
                },
                EitherAttributes::B(this) => match state {
                    EitherState::B(state) => B::render_update(this, renderer, state),
                    EitherState::A(old_state) => {
                        old_state.state_unmount(renderer);
                        *state = EitherState::B(this.render_init(renderer));
                    }
                },
            }
        }
    }
}
#[cfg(feature = "ssr")]
mod ssr {
    use async_str_iter::either::IterEither;

    use crate::{ssr::SsrAttributes, values::EitherAttributes};

    impl<A: SsrAttributes, B: SsrAttributes> SsrAttributes for EitherAttributes<A, B> {
        type IntoSsrAttributes = IterEither<A::IntoSsrAttributes, B::IntoSsrAttributes>;

        fn into_ssr_attributes(self) -> Self::IntoSsrAttributes {
            match self {
                EitherAttributes::A(this) => IterEither::Left(this.into_ssr_attributes()),
                EitherAttributes::B(this) => IterEither::Right(this.into_ssr_attributes()),
            }
        }
    }
}
