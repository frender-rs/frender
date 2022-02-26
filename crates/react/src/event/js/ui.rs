use js_sys::Object;
use wasm_bindgen::prelude::*;

use super::BaseSyntheticEvent;

#[wasm_bindgen]
extern "C" {
    /// React synthetic UiEvent
    #[wasm_bindgen(extends = Object, extends = BaseSyntheticEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type UiEvent;

    #[wasm_bindgen(method, getter)]
    pub fn detail(this: &UiEvent) -> i32;
    #[wasm_bindgen(method, getter)]
    pub fn view(this: &UiEvent) -> i32;
}
