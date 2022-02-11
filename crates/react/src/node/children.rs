use wasm_bindgen::JsValue;

use crate::AnyNode;

#[derive(Debug, Clone)]
pub struct Children {
    children: js_sys::Array,
}

impl Children {
    pub(crate) fn from_js_array(arr: js_sys::Array) -> Self {
        Self { children: arr }
    }

    pub fn new() -> Self {
        let children = js_sys::Array::new();
        Self { children }
    }

    pub fn chain_one<N: crate::Node>(self, node: N) -> Self {
        self.children.push(&node.as_react_node_js().0);
        self
    }

    pub fn chain<T: IntoIterator<Item = AnyNode>>(mut self, nodes: T) -> Self {
        self.children = self.children.concat(&js_sys::Array::from_iter(nodes));
        self
    }
}

impl super::Node for Children {
    fn as_react_node_js(&self) -> AnyNode {
        AnyNode(react_sys::create_fragment(&JsValue::NULL, &self.children).into())
    }

    fn as_react_children_js(&self) -> Option<Children> {
        Some(self.clone())
    }
}

impl AsRef<JsValue> for Children {
    fn as_ref(&self) -> &JsValue {
        self.children.as_ref()
    }
}

impl AsRef<js_sys::Array> for Children {
    fn as_ref(&self) -> &js_sys::Array {
        &self.children
    }
}

impl Into<js_sys::Array> for Children {
    #[inline]
    fn into(self) -> js_sys::Array {
        self.children
    }
}

impl FromIterator<AnyNode> for Children {
    fn from_iter<T: IntoIterator<Item = AnyNode>>(iter: T) -> Self {
        let children = js_sys::Array::from_iter(iter);
        Self { children }
    }
}
