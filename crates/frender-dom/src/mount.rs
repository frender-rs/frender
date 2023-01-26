use frender_core::UpdateRenderState;

use crate::Dom;

pub fn mount_to_dom_element_by_id<E: UpdateRenderState<Dom> + Copy>(
    element: E,
    id: &str,
) -> impl std::future::Future<Output = ()> {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = document.get_element_by_id(&id).unwrap();

    async move {
        Dom::new(document, current_parent)
            .render_element(element, std::future::pending())
            .await
    }
}

#[cfg(feature = "spawn")]
pub fn spawn_mount_to_dom_element_by_id<E: UpdateRenderState<Dom> + Copy + 'static>(
    element: E,
    id: &str,
) {
    wasm_bindgen_futures::spawn_local(mount_to_dom_element_by_id(element, id))
}
