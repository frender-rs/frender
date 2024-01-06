use crate::{html::behavior_type_traits::Element as ElementType, RenderHtml};

pub trait UpdateElementNonReactive<ET: ElementType> {
    type State<Renderer: RenderHtml + ?Sized>: Default;

    fn update_element_non_reactive<Renderer: RenderHtml + ?Sized>(this: Self, renderer: &mut Renderer, element: &mut ET::Node<Renderer>, state: &mut Self::State<Renderer>);
}

impl<ET: ElementType> UpdateElementNonReactive<ET> for () {
    type State<Renderer: RenderHtml + ?Sized> = ();

    fn update_element_non_reactive<Renderer: RenderHtml + ?Sized>(_: Self, _: &mut Renderer, _: &mut ET::Node<Renderer>, _: &mut Self::State<Renderer>) {}
}

impl<ET: ElementType, A: UpdateElementNonReactive<ET>, B: UpdateElementNonReactive<ET>> UpdateElementNonReactive<ET> for (A, B) {
    type State<Renderer: RenderHtml + ?Sized> = (A::State<Renderer>, B::State<Renderer>);

    fn update_element_non_reactive<Renderer: RenderHtml + ?Sized>((a, b): Self, renderer: &mut Renderer, element: &mut ET::Node<Renderer>, (state_a, state_b): &mut Self::State<Renderer>) {
        A::update_element_non_reactive(a, renderer, element, state_a);
        B::update_element_non_reactive(b, renderer, element, state_b);
    }
}
