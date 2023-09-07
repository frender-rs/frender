use crate::{
    renderer::{MarkPositionAfter, MarkPositionAtFirstChild, RemoveNode},
    RenderHtml,
};

pub mod element_types {
    use crate::ElementType;

    pub enum Element {}

    impl ElementType for Element {
        type Element<Renderer: crate::RenderHtml> = Renderer::Element;

        fn remove_from_renderer<Renderer: crate::RenderHtml>(
            renderer: &mut Renderer,
            element: &mut Self::Element<Renderer>,
        ) {
            renderer.remove_node(element)
        }
    }
}

pub mod element_do_for_renderer {
    use crate::renderer::{MarkPositionAfter, MarkPositionAtFirstChild, RemoveNode};

    pub trait RemoveFromRenderer<R: ?Sized> {
        fn remove_from_renderer(&mut self, renderer: &mut R);
    }

    impl<E, R: ?Sized + RemoveNode<E>> RemoveFromRenderer<R> for E {
        fn remove_from_renderer(&mut self, renderer: &mut R) {
            renderer.remove_node(self)
        }
    }

    pub trait MarkPositionAfterSelfForRenderer<R: ?Sized> {
        fn mark_position_after_self_for_renderer(&mut self, renderer: &mut R);
    }

    impl<E, R: ?Sized + MarkPositionAfter<E>> MarkPositionAfterSelfForRenderer<R> for E {
        fn mark_position_after_self_for_renderer(&mut self, renderer: &mut R) {
            renderer.mark_position_after(self)
        }
    }

    pub trait MarkPositionAtFirstChildForRenderer<R: ?Sized> {
        fn mark_position_at_first_child_for_renderer(&mut self, renderer: &mut R);
    }

    impl<E, R: ?Sized + MarkPositionAtFirstChild<E>> MarkPositionAtFirstChildForRenderer<R> for E {
        fn mark_position_at_first_child_for_renderer(&mut self, renderer: &mut R) {
            renderer.mark_position_at_first_child(self)
        }
    }
}

pub trait ElementType {
    type Element<Renderer: RenderHtml>: element_do_for_renderer::RemoveFromRenderer<Renderer>
        + element_do_for_renderer::MarkPositionAfterSelfForRenderer<Renderer>
        + element_do_for_renderer::MarkPositionAtFirstChildForRenderer<Renderer>;

    fn remove_from_renderer<Renderer: RenderHtml>(
        renderer: &mut Renderer,
        element: &mut Self::Element<Renderer>,
    );
}

pub type ElementOfType<ET, R> = <ET as ElementType>::Element<R>;

pub trait UpdateElementNonReactive<ET: ElementType> {
    type State: Default;

    fn update_element_non_reactive<Renderer: RenderHtml>(
        this: Self,
        renderer: &mut Renderer,
        element: &mut ElementOfType<ET, Renderer>,
        state: &mut Self::State,
    );
}

impl<ET: ElementType> UpdateElementNonReactive<ET> for () {
    type State = ();

    fn update_element_non_reactive<Renderer: RenderHtml>(
        _: Self,
        _: &mut Renderer,
        _: &mut ElementOfType<ET, Renderer>,
        _: &mut Self::State,
    ) {
    }
}

impl<ET: ElementType, A: UpdateElementNonReactive<ET>, B: UpdateElementNonReactive<ET>>
    UpdateElementNonReactive<ET> for (A, B)
{
    type State = (A::State, B::State);

    fn update_element_non_reactive<Renderer: RenderHtml>(
        (a, b): Self,
        renderer: &mut Renderer,
        element: &mut ElementOfType<ET, Renderer>,
        (state_a, state_b): &mut Self::State,
    ) {
        A::update_element_non_reactive(a, renderer, element, state_a);
        B::update_element_non_reactive(b, renderer, element, state_b);
    }
}

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;

    type ElementType: ElementType;
}

mod demo {
    // use crate::UpdateElementNonReactive;

    struct id(pub String);

    pub trait UpdateElementNonReactive<E> {
        type State;

        fn update_element_non_reactive<Renderer: crate::RenderHtml>(
            this: Self,
            renderer: &mut Renderer,
            element: &mut E,
            state: &mut Self::State,
        );
    }

    impl<E> UpdateElementNonReactive<E> for id {
        type State = ();

        fn update_element_non_reactive<Renderer: crate::RenderHtml>(
            this: Self,
            renderer: &mut Renderer,
            element: &mut E,
            state: &mut Self::State,
        ) {
            todo!()
        }
    }
}
