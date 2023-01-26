use std::pin::Pin;

use frender_dom::Dom;

use super::IntrinsicComponentPollReactive;

pub trait UpdateElement<E> {
    type State: Default + IntrinsicComponentPollReactive;

    fn update_element(
        this: Self,
        element: &E,
        children_ctx: &mut Dom,
        state: Pin<&mut Self::State>,
    );
}
