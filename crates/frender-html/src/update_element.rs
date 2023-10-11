use crate::{
    renderer::{node_behaviors, ElementType},
    RenderHtml,
};

// pub trait ElementType {
//     type Element<Renderer: RenderHtml>: node_behaviors::Element<Renderer>;
// }

// pub trait HtmlElementType: ElementType {
//     type HtmlElement<Renderer: RenderHtml>: node_behaviors::HtmlElement<Renderer>
//         + Identity<This = Self::Element<Renderer>>;
// }

pub trait Identity {
    type This: ?Sized;
    fn from_identity_mut(this: &mut Self::This) -> &mut Self;
}

impl<T: ?Sized> Identity for T {
    type This = T;
    fn from_identity_mut(v: &mut T) -> &mut Self {
        v
    }
}

pub type ElementOfType<ET, R> = <ET as ElementType>::Element<R>;

pub trait UpdateElementNonReactive<ET: ElementType> {
    type State<Renderer: RenderHtml>: Default;

    fn update_element_non_reactive<Renderer: RenderHtml>(
        this: Self,
        renderer: &mut Renderer,
        element: &mut ElementOfType<ET, Renderer>,
        state: &mut Self::State<Renderer>,
    );
}

impl<ET: ElementType> UpdateElementNonReactive<ET> for () {
    type State<Renderer: RenderHtml> = ();

    fn update_element_non_reactive<Renderer: RenderHtml>(
        _: Self,
        _: &mut Renderer,
        _: &mut ElementOfType<ET, Renderer>,
        _: &mut Self::State<Renderer>,
    ) {
    }
}

impl<ET: ElementType, A: UpdateElementNonReactive<ET>, B: UpdateElementNonReactive<ET>>
    UpdateElementNonReactive<ET> for (A, B)
{
    type State<Renderer: RenderHtml> = (A::State<Renderer>, B::State<Renderer>);

    fn update_element_non_reactive<Renderer: RenderHtml>(
        (a, b): Self,
        renderer: &mut Renderer,
        element: &mut ElementOfType<ET, Renderer>,
        (state_a, state_b): &mut Self::State<Renderer>,
    ) {
        A::update_element_non_reactive(a, renderer, element, state_a);
        B::update_element_non_reactive(b, renderer, element, state_b);
    }
}

pub trait IntrinsicComponent {
    const INTRINSIC_TAG: &'static str;

    type ElementType: ElementType;
    type ElementTagType: ElementTagType;
}

pub trait ElementTagType {}

pub trait ElementSupportChildren<C>: ElementTagType {
    type ChildrenRenderState<R: RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<R: RenderHtml>(
        children: C,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
    );
}

pub mod element_tag_types {
    use crate::Element;

    pub enum EncloseAnyElement {}
    impl super::ElementTagType for EncloseAnyElement {}
    impl<E: Element> super::ElementSupportChildren<E> for EncloseAnyElement {
        type ChildrenRenderState<R: crate::RenderHtml> = E::RenderState<R>;

        fn children_render_update<R: crate::RenderHtml>(
            children: E,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
        ) {
            children.render_update(renderer, children_state)
        }
    }
}

mod demo {
    use crate::{
        renderer::{
            node_behaviors::{Element, HtmlElement},
            ElementType, HtmlElementType,
        },
        RenderHtml,
    };

    use super::{ElementOfType, Identity, UpdateElementNonReactive};

    struct id(pub String);

    impl<ET: ElementType> UpdateElementNonReactive<ET> for id {
        type State<Renderer: RenderHtml> = ();

        fn update_element_non_reactive<Renderer: crate::RenderHtml>(
            this: Self,
            renderer: &mut Renderer,
            element: &mut ElementOfType<ET, Renderer>,
            state: &mut Self::State<Renderer>,
        ) {
            element.set_id(renderer, &this.0)
        }
    }

    struct access_key(pub String);

    impl<ET: HtmlElementType> UpdateElementNonReactive<ET> for access_key {
        type State<Renderer: RenderHtml> = ();

        fn update_element_non_reactive<Renderer: crate::RenderHtml>(
            this: Self,
            renderer: &mut Renderer,
            element: &mut ElementOfType<ET, Renderer>,
            state: &mut Self::State<Renderer>,
        ) {
            let element = <ET::HtmlElement<Renderer> as Identity>::from_identity_mut(element);
            element.set_access_key(renderer, &this.0)
        }
    }
}
