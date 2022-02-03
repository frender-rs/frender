// mod counter_comp;
mod timer_comp;
mod utils;

// use counter_comp::DivProps;
use frender::react;
use wasm_bindgen::prelude::*;

pub use timer_comp::{Timer, TimerClosure, TimerJs};

pub use utils::set_panic_hook;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = ReactDOM)]
    // fn render(react_element: react::sys::Element, dom_element: web_sys::Element);

    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    fn create_element_fn(
        element_type: &Closure<dyn Fn(js_sys::Object) -> react::sys::Element>,
        props: JsValue,
        children: js_sys::Array,
    ) -> react::sys::Element;

    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    fn create_element_fn_mut(
        element_type: &Closure<dyn FnMut(js_sys::Object) -> react::sys::Element>,
        props: JsValue,
        children: js_sys::Array,
    ) -> react::sys::Element;

    #[wasm_bindgen(variadic, js_namespace = React, js_name = createElement)]
    fn create_element_html(
        element_type: &str,
        props: &JsValue,
        children: &js_sys::Array,
    ) -> react::sys::Element;
}

// #[wasm_bindgen(start)]
// pub fn start_react() {
//     let window = web_sys::window().unwrap();
//     let document = window.document().unwrap();

//     let el = document.get_element_by_id("root-rust").unwrap();

//     // let react_element =
//     //     TimerJs.with(|comp| react::sys::create_element(comp, JsValue::NULL, js_sys::Array::new()));

//     let react_element =
//         TimerClosure.with(|comp| create_element_fn(comp, JsValue::NULL, js_sys::Array::new()));

//     let react_element = counter_comp::div(DivProps {
//         children: [
//             //
//             react_element,
//             JsValue::from_str("child 2").into(),
//         ]
//         .into(),
//     });
//     // react_sys::react_dom::
//     react::sys::react_dom::render(&react_element, &el);
// }
