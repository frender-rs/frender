use frender_core::UpdateRenderState;

use crate::Dom;

pub async fn mount_to_dom_element_by_id<E: UpdateRenderState<Dom> + Copy>(
    element: E,
    id: std::borrow::Cow<'_, str>,
) {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = document.get_element_by_id(&id).unwrap();

    Dom::new(document, current_parent)
        .render_element(element, std::future::pending())
        .await;
}

#[cfg(feature = "spawn")]
pub fn spawn_mount_to_dom_element_by_id<E: UpdateRenderState<Dom> + Copy + 'static>(
    element: E,
    id: &str,
) {
    let window = web_sys::window().unwrap();

    let document = window.document().unwrap();
    let current_parent = document.get_element_by_id(id).unwrap();
    let mut dom = Dom::new(document, current_parent);

    wasm_bindgen_futures::spawn_local(async move {
        web_sys::console::log_1(&"start".into());
        dom.render_element(element, std::future::pending()).await
    })
}
