use frender_dom::component::IntrinsicElementType as ElementTagType;

use crate::{
    html::behavior_type_traits::Element as ElementType,
    renderer::{node_behaviors, CreateNode},
    RenderHtml,
};

pub trait Identity {
    type This: ?Sized;
    fn from_identity_mut(this: &mut Self::This) -> &mut Self;
    fn from_identity(this: Self::This) -> Self
    where
        Self: Sized;
}

impl<T: ?Sized> Identity for T {
    type This = T;
    fn from_identity_mut(v: &mut T) -> &mut Self {
        v
    }

    fn from_identity(this: Self::This) -> Self
    where
        Self: Sized,
    {
        this
    }
}

pub type ElementOfType<ET, R> = <ET as ElementType>::Element<R>;

pub trait UpdateElementNonReactive<ET: ElementType> {
    type State<Renderer: RenderHtml>: Default;

    fn update_element_non_reactive<Renderer: RenderHtml>(this: Self, renderer: &mut Renderer, element: &mut ET::Node<Renderer>, state: &mut Self::State<Renderer>);
}

impl<ET: ElementType> UpdateElementNonReactive<ET> for () {
    type State<Renderer: RenderHtml> = ();

    fn update_element_non_reactive<Renderer: RenderHtml>(_: Self, _: &mut Renderer, _: &mut ET::Node<Renderer>, _: &mut Self::State<Renderer>) {}
}

impl<ET: ElementType, A: UpdateElementNonReactive<ET>, B: UpdateElementNonReactive<ET>> UpdateElementNonReactive<ET> for (A, B) {
    type State<Renderer: RenderHtml> = (A::State<Renderer>, B::State<Renderer>);

    fn update_element_non_reactive<Renderer: RenderHtml>((a, b): Self, renderer: &mut Renderer, element: &mut ET::Node<Renderer>, (state_a, state_b): &mut Self::State<Renderer>) {
        A::update_element_non_reactive(a, renderer, element, state_a);
        B::update_element_non_reactive(b, renderer, element, state_b);
    }
}
