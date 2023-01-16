use std::pin::Pin;

use frender_dom::Dom;

pub trait UpdateElement<E> {
    type State: Default;

    fn update_element(self, element: E, dom: &mut Dom, state: Pin<Self::State>);
}
