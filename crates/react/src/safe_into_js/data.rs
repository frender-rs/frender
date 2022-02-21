use std::any::Any;
use wasm_bindgen::JsValue;

pub struct PassedToJsRuntime {
    pub js_value: JsValue,
    pub to_persist: Option<Box<dyn Any>>,
}

pub trait SafeIntoJsRuntime: Sized {
    fn safe_into_js_runtime(self) -> PassedToJsRuntime;
}

impl<T: SafeIntoJsRuntime> SafeIntoJsRuntime for Box<T> {
    fn safe_into_js_runtime(self) -> PassedToJsRuntime {
        (*self).safe_into_js_runtime()
    }
}
