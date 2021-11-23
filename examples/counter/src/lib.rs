mod timer_comp;
mod utils;

use frender::react_sys;
use wasm_bindgen::prelude::*;

pub use timer_comp::{Timer, TimerJs};

pub use utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ReactDOM)]
    fn render(react_element: react_sys::Element, dom_element: web_sys::Element);
}

#[wasm_bindgen(start)]
pub fn start_react() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let el = document.get_element_by_id("root-rust").unwrap();

    let react_element =
        TimerJs.with(|comp| react_sys::create_element(comp, JsValue::NULL, js_sys::Array::new()));

    // react_sys::react_dom::
    render(react_element, el);
}
