use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type BaseSyntheticEvent;

    #[wasm_bindgen(method, getter = nativeEvent)]
    pub fn native_event_raw(this: &BaseSyntheticEvent) -> JsValue;
    #[wasm_bindgen(method, getter = currentTarget)]
    pub fn current_target_raw(this: &BaseSyntheticEvent) -> JsValue;
    #[wasm_bindgen(method, getter)]
    pub fn target(this: &BaseSyntheticEvent) -> super::native::EventTarget;
    #[wasm_bindgen(method, getter)]
    pub fn bubbles(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method, getter)]
    pub fn cancelable(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method, getter = defaultPrevented)]
    pub fn default_prevented(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method, getter = eventPhase)]
    pub fn event_phase(this: &BaseSyntheticEvent) -> f64;
    #[wasm_bindgen(method, getter = isTrusted)]
    pub fn is_trusted(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method, js_name = preventDefault)]
    pub fn prevent_default(this: &BaseSyntheticEvent);
    #[wasm_bindgen(method, js_name = isDefaultPrevented)]
    pub fn is_default_prevented(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method, js_name = stopPropagation)]
    pub fn stop_propagation(this: &BaseSyntheticEvent);
    #[wasm_bindgen(method, js_name = isPropagationStopped)]
    pub fn is_propagation_stopped(this: &BaseSyntheticEvent) -> bool;
    #[wasm_bindgen(method)]
    pub fn persist(this: &BaseSyntheticEvent);
    #[wasm_bindgen(method, getter = timeStamp)]
    pub fn time_stamp(this: &BaseSyntheticEvent) -> f64;
    #[wasm_bindgen(method, getter = type)]
    pub fn kind(this: &BaseSyntheticEvent) -> String;
}
