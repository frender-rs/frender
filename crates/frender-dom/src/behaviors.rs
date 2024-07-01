use crate::render::RenderWithContext;

pub trait NodeRenderSelf<Renderer: ?Sized + RenderWithContext> {
    /// Should create the node,
    /// add the node to dom at the cursor,
    /// and move the cursor after the node.
    fn render_self(render_context: &mut Renderer::RenderContext<'_>) -> Self;
}

pub trait NodeWithRenderContextAfterSelf<Renderer: ?Sized + RenderWithContext> {
    fn with_render_context_after_self<Res>(
        &mut self,
        renderer: &mut Renderer,
        f: impl FnOnce(&mut Renderer::RenderContext<'_>) -> Res,
    ) -> Res;
}

pub trait Node<Renderer: ?Sized> {
    fn log_self(&self, renderer: &mut Renderer);

    /// Should move the node if `force_reposition`,
    /// and move cursor after the node.
    fn readd_self(
        &mut self,
        render_context: &mut Renderer::RenderContext<'_>,
        force_reposition: bool,
    ) where
        Renderer: crate::render::RenderWithContext;

    fn cursor_is_at_self(&self, render_context: &Renderer::RenderContext<'_>) -> bool
    where
        Renderer: crate::render::RenderWithContext;

    fn check_and_move_cursor_after_self(&self, render_context: &mut Renderer::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext;

    fn remove_self(&mut self, renderer: &mut Renderer);
}

pub trait Element<Renderer: ?Sized>: Node<Renderer> {
    fn set_attribute(&mut self, renderer: &mut Renderer, name: &str, value: &str);
    fn remove_attribute(&mut self, renderer: &mut Renderer, name: &str);

    fn set_inner_html(&mut self, renderer: &mut Renderer, value: &str);

    /// This kind of method of behavior traits have the same name `as_node_ref`
    /// so that callers can call it with `$TraitName::as_node_ref` in macros.
    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::Element);
}

pub trait HtmlElement<Renderer: ?Sized>: Element<Renderer> {
    fn set_inner_text(&mut self, renderer: &mut Renderer, value: &str);

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement);
}

pub trait ElementWithClassList<Renderer: ?Sized>: Element<Renderer> {
    type ClassList<'a>: frender_html_common::DomTokenList
    where
        Self: 'a,
        Renderer: 'a;
    fn class_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::ClassList<'a>;
}

pub trait ElementWithRelList<Renderer: ?Sized>: Element<Renderer> {
    type RelList<'a>: ::frender_html_common::DomTokenList
    where
        Self: 'a,
        Renderer: 'a;
    fn rel_list<'a>(&'a mut self, renderer: &'a mut Renderer) -> Self::RelList<'a>;
}

pub trait ElementWithChildren<Renderer: ?Sized> {
    fn with_render_context_at_first_child_of_self<R>(
        &mut self,
        renderer: &mut Renderer,
        f: impl FnOnce(&mut Renderer::RenderContext<'_>) -> R,
    ) -> R
    where
        Renderer: crate::render::RenderWithContext;
}

#[cfg(feature = "web")]
impl<N: AsRef<web_sys::Node>, Renderer: ?Sized + crate::csr::web::Renderer> Node<Renderer>
    for crate::csr::web::Node<N>
{
    fn log_self(&self, _: &mut Renderer) {
        web_sys::console::log_1(self.0.as_ref());
    }

    fn cursor_is_at_self(&self, render_context: &Renderer::RenderContext<'_>) -> bool
    where
        Renderer: crate::render::RenderWithContext,
    {
        Renderer::cursor_is_at_node(render_context, self.0.as_ref())
    }

    fn check_and_move_cursor_after_self(&self, render_context: &mut <Renderer>::RenderContext<'_>)
    where
        Renderer: crate::render::RenderWithContext,
    {
        Renderer::check_and_move_cursor_after_node(render_context, self.0.as_ref())
    }

    fn readd_self(
        &mut self,
        render_context: &mut Renderer::RenderContext<'_>,
        force_reposition: bool,
    ) where
        Renderer: crate::render::RenderWithContext,
    {
        Renderer::readd_node(render_context, self.0.as_ref(), force_reposition)
    }

    fn remove_self(&mut self, renderer: &mut Renderer) {
        renderer.remove_node(self.0.as_ref())
    }
}

#[cfg(feature = "web")]
impl<
        N: AsRef<web_sys::Node> + AsRef<web_sys::Element>,
        Renderer: ?Sized + crate::csr::web::Renderer,
    > Element<Renderer> for crate::csr::web::Node<N>
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

    fn set_inner_html(&mut self, _: &mut Renderer, value: &str) {
        AsRef::<web_sys::Element>::as_ref(&self.0).set_inner_html(value)
    }

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::Element) {
        AsRef::<web_sys::Element>::as_ref(&self.0)
    }
}

#[cfg(feature = "web")]
impl<
        N: AsRef<web_sys::Node> + AsRef<web_sys::Element> + AsRef<web_sys::HtmlElement>,
        Renderer: ?Sized + crate::csr::web::Renderer,
    > HtmlElement<Renderer> for crate::csr::web::Node<N>
{
    fn set_inner_text(&mut self, _: &mut Renderer, value: &str) {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.0).set_inner_text(value)
    }

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement) {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.0)
    }
}

#[cfg(feature = "web")]
mod web {
    use crate::shims::RelList as _;

    use super::*;

    impl<
            N: AsRef<web_sys::Node> + AsRef<web_sys::Element>,
            Renderer: ?Sized + crate::csr::web::Renderer,
        > ElementWithChildren<Renderer> for crate::csr::web::Node<N>
    {
        fn with_render_context_at_first_child_of_self<R>(
            &mut self,
            renderer: &mut Renderer,
            f: impl FnOnce(&mut <Renderer as crate::render::RenderWithContext>::RenderContext<'_>) -> R,
        ) -> R {
            renderer.with_render_context_at_first_child_of_element(self.0.as_ref(), f)
        }
    }

    impl<
            N: AsRef<web_sys::Node> + AsRef<web_sys::Element>,
            Renderer: ?Sized + crate::csr::web::Renderer,
        > ElementWithClassList<Renderer> for crate::csr::web::Node<N>
    {
        type ClassList<'a> = crate::csr::web::DomTokenList
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
            type RelList<'a> = crate::csr::web::DomTokenList
            where
                Self: 'a,
                Renderer: 'a;
            fn rel_list<'a>(&'a mut self, _: &'a mut Renderer) -> Self::RelList<'a> {
                self.0.rel_list().into()
            }
        }
    );
}
