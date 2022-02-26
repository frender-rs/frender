pub trait RenderIntoDom {
    fn render_into_dom(self, dom_element: &web_sys::Element);
}

impl<E: crate::IntoElement> RenderIntoDom for E {
    #[inline]
    fn render_into_dom(self, dom_element: &web_sys::Element) {
        let el = self.into_element();
        let el = el.unsafe_into_js_element();
        react_sys::react_dom::render(&el, dom_element);
        // TODO: if need to persist
        // std::mem::forget(el);
    }
}

pub fn render_into_dom_by_id<E: RenderIntoDom>(frender_element: E, dom_element_id: &str) {
    use wasm_bindgen::UnwrapThrowExt;
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let el = document
        .get_element_by_id(dom_element_id)
        .expect_throw("No element is found with getElementById");

    frender_element.render_into_dom(&el);
}
