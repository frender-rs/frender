use std::collections::HashMap;

use wasm_bindgen::JsValue;

pub struct CssProperties {
    custom: Option<HashMap<String, String>>,
}
