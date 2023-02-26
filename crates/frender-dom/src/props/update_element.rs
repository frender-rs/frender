use std::pin::Pin;

use crate::Dom;

pub trait UpdateElement<E> {
    type State: IntrinsicComponentPollReactive;

    fn initialize_state(this: Self, element: &E, children_ctx: &mut Dom) -> Self::State;

    fn update_element(
        this: Self,
        element: &E,
        children_ctx: &mut Dom,
        state: Pin<&mut Self::State>,
    );
}

pub trait IntrinsicComponentPollReactive {
    fn intrinsic_component_poll_reactive(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<bool>;
}
