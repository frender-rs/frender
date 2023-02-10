use std::pin::Pin;

use frender_dom::Dom;

use super::IntrinsicComponentPollReactive;

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
