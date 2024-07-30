pub use frender_html::{CsrComponent, CsrElement, RenderState};
pub use into_render_element_ext::IntoRenderElementExt;
pub use render_element::RenderElement;

mod render_element;

mod into_render_element_ext {
    use frender_html::{dom::ProvideRenderContext, CsrElement as Element, RenderHtml};

    pub trait IntoRenderElementExt: ProvideRenderContext {
        fn into_render_element<E: Element>(self, element: E) -> crate::RenderElement<Self, E>
        where
            Self: Sized,
            Self::Renderer: RenderHtml,
        {
            crate::RenderElement::new(self, element)
        }

        fn render_element<E: Element>(&mut self, element: E) -> crate::RenderElement<&mut Self, E>
        where
            Self::Renderer: RenderHtml,
        {
            crate::RenderElement::new(self, element)
        }
    }

    impl<R: ?Sized + ProvideRenderContext> IntoRenderElementExt for R {}
}
