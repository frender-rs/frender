use wasm_bindgen::JsValue;

use crate::AnyNode;

#[derive(Debug, Clone)]
pub enum Children {
    Single(AnyNode),
    StaticMultiple(js_sys::Array),
}

impl Children {
    pub fn from_static_nodes<T: IntoIterator<Item = AnyNode>>(nodes: T) -> Self {
        Self::StaticMultiple(js_sys::Array::from_iter(nodes))
    }
}

impl super::Node for Children {
    fn as_react_node_js(&self) -> AnyNode {
        match self {
            Children::Single(node) => node.clone(),
            Children::StaticMultiple(arr) => {
                AnyNode(react_sys::create_fragment(&JsValue::NULL, arr).into())
            }
        }
    }

    fn as_react_children_js(&self) -> Option<Children> {
        Some(self.clone())
    }
}

impl AsRef<JsValue> for Children {
    fn as_ref(&self) -> &JsValue {
        match self {
            Children::Single(node) => node.as_ref(),
            Children::StaticMultiple(arr) => arr.as_ref(),
        }
    }
}
