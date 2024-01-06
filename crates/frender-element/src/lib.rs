pub use frender_html::{CsrComponent, Element, RenderState};
pub use into_render_element_ext::IntoRenderElementExt;
pub use render_element::RenderElement;

mod render_element;

mod into_render_element_ext {
    use frender_html::RenderHtml;

    pub trait IntoRenderElementExt: RenderHtml {
        fn into_render_element<E, S: crate::RenderState<E, Self> + Default>(
            self,
            root: E,
        ) -> crate::RenderElement<Self, E, S>
        where
            Self: Sized,
        {
            crate::RenderElement::new(self, root)
        }
    }

    impl<R: ?Sized + RenderHtml> IntoRenderElementExt for R {}
}
