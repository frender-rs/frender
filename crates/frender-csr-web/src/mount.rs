use frender_element::Element;

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

pub fn mount_to_dom_element<'e, E: Element + 'e>(
    element: E,
    get_dom_element: impl GetDomElement,
) -> impl std::future::Future<Output = ()> + 'e {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = get_dom_element.get_dom_element(&document);

    async move {
        crate::renderer::Renderer::new(&document, current_parent)
            .render_element(element)
            .await
    }
}

#[cfg(feature = "spawn")]
pub fn spawn_mount_to_dom_element<E: Element + 'static>(
    get_element: E,
    get_dom_element: impl GetDomElement,
) {
    wasm_bindgen_futures::spawn_local(mount_to_dom_element(get_element, get_dom_element))
}
