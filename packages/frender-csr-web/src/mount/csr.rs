use frender_csr_ext::IntoRenderElementExt as _;
use frender_html::csr::CsrElement;

use super::GetDomElement;

pub fn mount_to_dom_element<'e, E: CsrElement + 'e>(
    element: E,
    get_dom_element: impl GetDomElement,
) -> impl std::future::Future<Output = ()> + 'e {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = get_dom_element.get_dom_element(&document);

    crate::renderer::RendererWithRoot::new(document, current_parent).into_render_element(element)
}

#[cfg(feature = "spawn")]
pub fn spawn_mount_to_dom_element<E: CsrElement + 'static>(
    get_element: E,
    get_dom_element: impl GetDomElement,
) {
    wasm_bindgen_futures::spawn_local(mount_to_dom_element(get_element, get_dom_element))
}
