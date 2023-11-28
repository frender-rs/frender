use frender_html::{dom::component::IntrinsicElementType as ElementTagType, RenderHtml};

pub trait ElementSupportChildren<C>: ElementTagType {
    type ChildrenRenderState<R: RenderHtml>: crate::RenderState<R> + Default;

    fn children_render_update<R: RenderHtml>(
        children: C,
        renderer: &mut R,
        children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>,
    );

    type ChildrenUnpinnedRenderState<R: RenderHtml>: crate::RenderState<R> + Default + Unpin;

    fn children_unpinned_render_update<R: RenderHtml>(
        children: C,
        renderer: &mut R,
        children_state: &mut Self::ChildrenUnpinnedRenderState<R>,
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

        type ChildrenUnpinnedRenderState<R: frender_html::RenderHtml> = E::UnpinnedRenderState<R>;

        fn children_unpinned_render_update<R: frender_html::RenderHtml>(
            children: E,
            renderer: &mut R,
            children_state: &mut Self::ChildrenUnpinnedRenderState<R>,
        ) {
            children.unpinned_render_update(renderer, children_state)
        }
    }
}
