pub trait Node<Renderer: ?Sized> {
    fn cursor_is_at_self(&self, renderer: &Renderer) -> bool;

    fn move_cursor_after_self(&mut self, renderer: &mut Renderer);

    /// should move cursor
    fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool);

    fn remove_self(&mut self, renderer: &mut Renderer);
}

pub trait Element<Renderer: ?Sized>: Node<Renderer> {
    fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer);

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

#[cfg(feature = "web")]
impl<N: AsRef<web_sys::Node>, Renderer: ?Sized + crate::csr::web::Renderer> Node<Renderer>
    for crate::csr::web::Node<N>
{
    fn cursor_is_at_self(&self, renderer: &Renderer) -> bool {
        renderer.cursor_is_at_node(self.0.as_ref())
    }

    fn move_cursor_after_self(&mut self, renderer: &mut Renderer) {
        renderer.move_cursor_after_node(self.0.as_ref())
    }

    fn readd_self(&mut self, renderer: &mut Renderer, force_reposition: bool) {
        renderer.readd_node(self.0.as_ref(), force_reposition)
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
    fn move_cursor_at_the_first_child_of_self(&mut self, renderer: &mut Renderer) {
        renderer.move_cursor_at_the_first_child_of_element(self.0.as_ref())
    }

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
