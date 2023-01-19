use std::pin::Pin;

use frender_dom::Dom;

pub trait UpdateElement<E> {
    type State;

    fn update_element(
        this: Self,
        element: &E,
        children_ctx: &mut Dom,
        state: Pin<&mut Self::State>,
    );
}
