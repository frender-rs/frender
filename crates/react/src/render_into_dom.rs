pub trait RenderIntoDom {
    fn render_into_dom(self, dom_element: &web_sys::Element);
}

impl<E: crate::IntoElement> RenderIntoDom for E {
    #[inline]
    fn render_into_dom(self, dom_element: &web_sys::Element) {
        let el = self.into_element();
        let el = el.unsafe_into_js_element();
        /// TODO: if need to persist
        react_sys::react_dom::render(&el, dom_element);
    }
}
