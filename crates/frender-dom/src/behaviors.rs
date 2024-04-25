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

    /// On drop, the event listener should be removed.
    type OnBeforeInputPreventDefault;
    fn on_before_input_prevent_default(
        &mut self,
        renderer: &mut Renderer,
    ) -> Self::OnBeforeInputPreventDefault;

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement);
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

    type OnBeforeInputPreventDefault =
        crate::csr::web::event::EventListenerPreventDefault<&'static str>;

    fn on_before_input_prevent_default(
        &mut self,
        _: &mut Renderer,
    ) -> Self::OnBeforeInputPreventDefault {
        let node: &web_sys::Node = self.0.as_ref();
        let node: &web_sys::EventTarget = node.as_ref();

        Self::OnBeforeInputPreventDefault::new(node.clone(), "beforeinput")
    }

    fn as_node_ref(&self) -> &(dyn 'static + crate::node_ref::traits::HtmlElement) {
        AsRef::<web_sys::HtmlElement>::as_ref(&self.0)
    }
}
