use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type MouseEvent;

    #[wasm_bindgen(method, getter = altKey)]
    pub fn alt_key(this: &MouseEvent) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn button(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter)]
    pub fn buttons(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = clientX)]
    pub fn client_x(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = clientY)]
    pub fn client_y(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = ctrlKey)]
    pub fn ctrl_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method,js_name = getModifierState)]
    pub fn get_modifier_state(this: &MouseEvent, key: &str) -> bool;

    #[wasm_bindgen(method, getter = metaKey)]
    pub fn meta_key(this: &MouseEvent) -> bool;

    #[wasm_bindgen(method, getter = movementX)]
    pub fn movement_x(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = movementY)]
    pub fn movement_y(this: &MouseEvent) -> f64;

    #[wasm_bindgen(method, getter = pageX)]
    pub fn page_x(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = pageY)]
    pub fn page_y(this: &MouseEvent) -> f64;

    #[wasm_bindgen(method, getter = relatedTarget)]
    pub fn related_target(this: &MouseEvent) -> Option<super::native::EventTarget>;

    #[wasm_bindgen(method, getter = screenX)]
    pub fn screen_x(this: &MouseEvent) -> f64;
    #[wasm_bindgen(method, getter = screenY)]
    pub fn screen_y(this: &MouseEvent) -> f64;

    #[wasm_bindgen(method, js_name = shiftKey)]
    pub fn shift_key(this: &MouseEvent) -> bool;
}
