use frender_core::UpdateRenderState;

use crate::Dom;

pub trait GetDomElement {
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element;
}

impl GetDomElement for &str {
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element {
        document
            .get_element_by_id(self)
            .expect("document should have an element with the specified id")
    }
}

impl<F> GetDomElement for F
where
    F: FnOnce(&web_sys::Document) -> web_sys::Element,
{
    #[inline(always)]
    fn get_dom_element(self, document: &web_sys::Document) -> web_sys::Element {
        self(document)
    }
}

pub fn mount_to_dom_element<'e, E: UpdateRenderState<Dom>>(
    get_element: impl FnMut() -> E + 'e,
    get_dom_element: impl GetDomElement,
) -> impl std::future::Future<Output = ()> + 'e {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = get_dom_element.get_dom_element(&document);

    async move {
        Dom::new(document, current_parent)
            .render_element(get_element, std::future::pending())
            .await
    }
}

#[cfg(feature = "spawn")]
pub fn spawn_mount_to_dom_element<E: UpdateRenderState<Dom> + 'static>(
    get_element: impl FnMut() -> E + 'static,
    get_dom_element: impl GetDomElement,
) {
    wasm_bindgen_futures::spawn_local(mount_to_dom_element(get_element, get_dom_element))
}