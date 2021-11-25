use wasm_bindgen::JsValue;

use super::Node;

pub struct AnyNode(Box<dyn Node>);

impl Node for AnyNode {
    fn as_react_node_js(&self) -> JsValue {
        self.0.as_react_node_js()
    }
}
