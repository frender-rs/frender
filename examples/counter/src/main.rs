use frender::node;
use frender::react::{self, Component};
use wasm_bindgen::UnwrapThrowExt;

fn component_strict_mode() -> wasm_bindgen::JsValue {
    let r = web_sys::window().unwrap().get("React").unwrap_throw();
    js_sys::Reflect::get(&r, &wasm_bindgen::JsValue::from_str("StrictMode")).unwrap_throw()
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let el = document.get_element_by_id("root-rust").unwrap();

    let value = 1;
    let react_element = node!(
        a[href="https://example.com"]{
            "example anchor"
            value
        }
    );

    let comp = component_strict_mode();

    let react_element = react::sys::create_element(
        &comp,
        &wasm_bindgen::JsValue::NULL,
        &js_sys::Array::of1(&react_element),
    );

    react::sys::react_dom::render(&react_element, &el);
}
