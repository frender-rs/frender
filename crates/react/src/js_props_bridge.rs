use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) type JsProps;
    pub(crate) type NodeFromJs;

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderPropsBridge")]
    pub fn props_bridge(this: &JsProps) -> Option<usize>;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderPropsBridge")]
    pub fn set_props_bridge(this: &JsProps, v: Option<usize>);

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderDebugComponentName")]
    pub fn debug_component_name(this: &JsProps) -> JsValue;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderDebugComponentName")]
    pub fn set_debug_component_name(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, getter, js_name = "__frenderDebugProps")]
    pub fn debug_props(this: &JsProps) -> JsValue;

    #[wasm_bindgen(structural, method, setter, js_name = "__frenderDebugProps")]
    pub fn set_debug_props(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, setter, js_name = "key")]
    fn _set_key(this: &JsProps, v: &JsValue);

    #[wasm_bindgen(structural, method, getter)]
    pub fn children(this: &JsProps) -> Option<NodeFromJs>;
}

impl JsProps {
    pub fn set_key(&self, v: Option<&JsValue>) {
        if let Some(v) = v {
            self._set_key(v);
        }
    }
}
