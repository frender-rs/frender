use frender_csr_core::render::RenderWithContext;

use crate::{csr::web, shims::RelList as _};

use super::{
    Element, ElementWithChildren, ElementWithClassList, ElementWithRelList, ElementWithStyle,
    HtmlElement, Node,
};

impl<N: AsRef<web_sys::Node>, Renderer: ?Sized + web::Renderer> Node<Renderer> for web::Node<N> {
    fn log_self(&self, _: &mut Renderer) {
        web_sys::console::log_1(self.0.as_ref());
    }

    fn warn_self_with_message(&self, _: &mut Renderer, message: &str) {
        web_sys::console::warn_2(self.0.as_ref(), &message.into());
    }

    fn cursor_is_at_self(&self, render_context: &Renderer::RenderContext<'_>) -> bool
    where
        Renderer: RenderWithContext,
    {
        Renderer::cursor_is_at_node(render_context, self.0.as_ref())
    }

    fn check_and_move_cursor_after_self(&self, render_context: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: RenderWithContext,
    {
        Renderer::check_and_move_cursor_after_node(render_context, self.0.as_ref())
    }

    fn readd_self(
        &mut self,
        render_context: &mut Renderer::RenderContext<'_>,
        force_reposition: bool,
    ) where
        Renderer: RenderWithContext,
    {
        Renderer::readd_node(render_context, self.0.as_ref(), force_reposition)
    }

    fn remove_self(&mut self, renderer: &mut Renderer) {
        renderer.remove_node(self.0.as_ref())
    }
}

impl<N: AsRef<web_sys::Node> + AsRef<web_sys::Element>, Renderer: ?Sized + web::Renderer>
    Element<Renderer> for web::Node<N>
{
    fn set_attribute(&mut self, _: &mut Renderer, name: &str, value: &str) {
        use wasm_bindgen::UnwrapThrowExt;

        AsRef::<web_sys::Element>::as_ref(&self.0)
            .set_attribute(name, value)
            .unwrap_throw()
    }

    fn remove_attribute(&mut self, _: &mut Renderer, name: &str) {
        use wasm_bindgen::UnwrapThrowExt;

        AsRef::<web_sys::Element>::as_ref(&self.0)
            .remove_attribute(name)
            .unwrap_throw()
    }

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::Element) {
        AsRef::<web_sys::Element>::as_ref(&self.0)
    }
}

impl<
        N: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
        Renderer: ?Sized + web::Renderer,
    > HtmlElement<Renderer> for web::Node<N>
{
    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement) {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.0)
    }
}

impl<N: AsRef<web_sys::Node> + AsRef<web_sys::Element>, Renderer: ?Sized + web::Renderer>
    ElementWithChildren<Renderer> for web::Node<N>
{
    fn with_render_context_at_first_child_of_self<R>(
        &mut self,
        renderer: &mut Renderer,
        f: impl FnOnce(&mut <Renderer as RenderWithContext>::RenderContext<'_>) -> R,
    ) -> R {
        renderer.with_render_context_at_first_child_of_element(self.0.as_ref(), f)
    }
}

impl<
        N: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
        Renderer: ?Sized + web::Renderer,
    > ElementWithStyle<Renderer> for web::Node<N>
{
    type Style<'a>
        = web_sys::CssStyleDeclaration
    where
        Self: 'a,
        Renderer: 'a;

    fn style<'a>(&'a mut self, _: &'a mut Renderer) -> Self::Style<'a> {
        web_sys::HtmlElement::style(self.0.as_ref())
    }
}

impl<N: AsRef<web_sys::Node> + AsRef<web_sys::Element>, Renderer: ?Sized + web::Renderer>
    ElementWithClassList<Renderer> for web::Node<N>
{
    type ClassList<'a>
        = web_sys::DomTokenList
    where
        Self: 'a,
        Renderer: 'a;

    fn class_list<'a>(&'a mut self, _: &'a mut Renderer) -> Self::ClassList<'a> {
        let element: &web_sys::Element = self.0.as_ref();
        element.class_list().into()
    }
}

frender_common::impl_many!(
    impl<__>
        (
            Generics![Renderer: ?Sized + crate::csr::web::Renderer],
            Trait![ElementWithRelList<Renderer>],
            each_of![
                crate::csr::web::Node<web_sys::HtmlAnchorElement>,
                crate::csr::web::Node<web_sys::HtmlAreaElement>,
                crate::csr::web::Node<web_sys::HtmlFormElement>,
                crate::csr::web::Node<web_sys::HtmlLinkElement>,
            ],
        )
    {
        type RelList<'a>
            = web_sys::DomTokenList
        where
            Self: 'a,
            Renderer: 'a;
        fn rel_list<'a>(&'a mut self, _: &'a mut Renderer) -> Self::RelList<'a> {
            self.0.rel_list().into()
        }
    }
);
