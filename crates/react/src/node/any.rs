use wasm_bindgen::JsValue;

use super::Node;

pub struct AnyNode(pub Box<dyn Node>);

impl AnyNode {
    pub fn wrap<N: 'static + Node>(node: N) -> Self {
        Self(Box::new(node))
    }
}

impl Node for AnyNode {
    fn as_react_node_js(&self) -> JsValue {
        self.0.as_react_node_js()
    }
}
