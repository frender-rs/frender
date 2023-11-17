// TODO: remove
use frender_html::RenderHtml;

pub use element::{Element, RenderState};
pub use element_types::ElementSupportChildren;
pub use into_render_element_ext::IntoRenderElementExt;
pub use render_element::RenderElement;

pub mod pin_mut_maybe_uninit;

mod element;

mod render_element;

mod element_types;

mod component;

pub mod elements;

mod into_render_element_ext {
    use frender_html::RenderHtml;

    pub trait IntoRenderElementExt: RenderHtml {
        fn into_render_element<S: crate::RenderState<Self> + Default>(
            self,
        ) -> crate::RenderElement<Self, S>
        where
            Self: Sized,
        {
            crate::RenderElement::new(self)
        }
    }

    impl<R: ?Sized + RenderHtml> IntoRenderElementExt for R {}
}
