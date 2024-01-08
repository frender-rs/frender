pub use frender_html::{CsrComponent, Element, RenderState};
pub use into_render_element_ext::IntoRenderElementExt;
pub use render_element::RenderElement;

mod render_element;

mod into_render_element_ext {
    use frender_html::{Element, RenderHtml};

    pub trait IntoRenderElementExt: RenderHtml {
        fn into_render_element<E: Element>(self, element: E) -> crate::RenderElement<Self, E>
        where
            Self: Sized,
        {
            crate::RenderElement::new(self, element)
        }
    }

    impl<R: ?Sized + RenderHtml> IntoRenderElementExt for R {}
}
