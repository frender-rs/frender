use wasm_bindgen::JsValue;

pub struct Children {
    children: js_sys::Array,
}

impl Children {
    pub fn new<const N: usize>(nodes: [JsValue; N]) -> Self {
        let children = js_sys::Array::from_iter(nodes);
        Self { children }
    }
}

impl super::Node for Children {
    fn as_react_node_js(&self) -> JsValue {
        react_sys::create_fragment(&JsValue::NULL, &self.children).into()
    }

    fn as_react_children_js(&self) -> Option<js_sys::Array> {
        Some(self.children.clone())
    }
}
