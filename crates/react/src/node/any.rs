use wasm_bindgen::JsValue;

use super::Node;

#[derive(Debug, Clone)]
pub struct AnyNode(pub JsValue);

impl AnyNode {
    #[inline]
    pub fn null() -> Self {
        Self(JsValue::NULL)
    }

    #[inline]
    pub fn wrap<N: Node>(node: N) -> Self {
        node.as_react_node_js()
    }
}

impl Node for AnyNode {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        self.clone()
    }

    #[inline]
    fn into_react_node_js(self) -> AnyNode
    where
        Self: Sized,
    {
        self
    }
}

impl AsRef<JsValue> for AnyNode {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}
