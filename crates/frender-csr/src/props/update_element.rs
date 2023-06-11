use std::pin::Pin;

use crate::{CsrContext, CsrRenderState};

pub trait UpdateElement<E> {
    type State: CsrRenderState;

    fn initialize_state(this: Self, element: &E, children_ctx: &mut CsrContext) -> Self::State;

    fn update_element(
        this: Self,
        element: &E,
        children_ctx: &mut CsrContext,
        state: Pin<&mut Self::State>,
    );
}

pub trait UpdateElementNonReactive<E> {
    type State;

    fn initialize_state_non_reactive(
        this: Self,
        element: &E,
        children_ctx: &mut CsrContext,
    ) -> Self::State;

    fn update_element_non_reactive(
        this: Self,
        element: &E,
        children_ctx: &mut CsrContext,
        state: Pin<&mut Self::State>,
    );
}

impl<E> UpdateElementNonReactive<E> for () {
    type State = ();

    fn initialize_state_non_reactive(_: Self, _: &E, _: &mut CsrContext) -> Self::State {}

    fn update_element_non_reactive(_: Self, _: &E, _: &mut CsrContext, _: Pin<&mut Self::State>) {}
}

impl<A, B, E> UpdateElementNonReactive<E> for (A, B)
where
    A: UpdateElementNonReactive<E>,
    B: UpdateElementNonReactive<E>,
{
    type State = (A::State, B::State);

    fn initialize_state_non_reactive(
        this: Self,
        element: &E,
        children_ctx: &mut CsrContext,
    ) -> Self::State {
        (
            A::initialize_state_non_reactive(this.0, element, children_ctx),
            B::initialize_state_non_reactive(this.1, element, children_ctx),
        )
    }

    fn update_element_non_reactive(
        this: Self,
        element: &E,
        children_ctx: &mut CsrContext,
        state: Pin<&mut Self::State>,
    ) {
        // SAFETY: pin projection
        let (a, b) = unsafe {
            match state.get_unchecked_mut() {
                (a, b) => (Pin::new_unchecked(a), Pin::new_unchecked(b)),
            }
        };

        A::update_element_non_reactive(this.0, element, children_ctx, a);
        B::update_element_non_reactive(this.1, element, children_ctx, b);
    }
}
