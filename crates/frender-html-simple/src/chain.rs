use std::pin::Pin;

pin_project_lite::pin_project! {
    pub struct ChainPropsState<A, B> {
        #[pin]
        a: A,
        #[pin]
        b: B,
    }
}

impl<A, B> ChainPropsState<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }

    pub fn pin_project(self: Pin<&mut Self>) -> (Pin<&mut A>, Pin<&mut B>) {
        let this = self.project();
        (this.a, this.b)
    }
}

pub struct ChainProps<A, B>(pub A, pub B);

#[cfg(feature = "dom")]
mod dom {
    use super::*;
    use frender_csr::{
        props::{IntrinsicComponentPollReactive, UpdateElement},
        Dom,
    };

    impl<A: IntrinsicComponentPollReactive, B: IntrinsicComponentPollReactive>
        IntrinsicComponentPollReactive for ChainPropsState<A, B>
    {
        fn intrinsic_component_poll_reactive(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<bool> {
            let this = self.project();
            frender_core::join_poll_reactive_results(
                this.a.intrinsic_component_poll_reactive(cx),
                this.b.intrinsic_component_poll_reactive(cx),
            )
        }
    }

    impl<E, A: UpdateElement<E>, B: UpdateElement<E>> UpdateElement<E> for ChainProps<A, B> {
        type State = ChainPropsState<A::State, B::State>;

        fn initialize_state(this: Self, element: &E, children_ctx: &mut Dom) -> Self::State {
            ChainPropsState::new(
                A::initialize_state(this.0, element, children_ctx),
                B::initialize_state(this.1, element, children_ctx),
            )
        }

        fn update_element(
            this: Self,
            element: &E,
            children_ctx: &mut Dom,
            state: std::pin::Pin<&mut Self::State>,
        ) {
            let state = state.project();
            A::update_element(this.0, element, children_ctx, state.a);
            B::update_element(this.1, element, children_ctx, state.b);
        }
    }
}
