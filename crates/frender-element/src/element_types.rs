use frender_html::{dom::component::IntrinsicElementType as ElementTagType, RenderHtml};

pub trait ElementSupportChildren<C>: ElementTagType {
    type ChildrenRenderState<R: RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<R: RenderHtml>(
        children: C,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
    );
}

mod element_types {
    use frender_html::dom::element_types::EncloseAnyElement;

    use crate::Element;

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
