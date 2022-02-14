use wasm_bindgen::JsValue;

pub trait DebugProps {
    fn as_debug_props(&self) -> Option<JsValue>;
}

impl<T: DebugProps> DebugProps for &T {
    fn as_debug_props(&self) -> Option<JsValue> {
        DebugProps::as_debug_props(*self)
    }
}
