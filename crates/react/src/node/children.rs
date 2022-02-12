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

    pub fn as_js_array(&self) -> std::borrow::Cow<js_sys::Array> {
        match self {
            Children::Single(node) => std::borrow::Cow::Owned(js_sys::Array::of1(node.as_ref())),
            Children::StaticMultiple(arr) => std::borrow::Cow::Borrowed(arr),
        }
    }
}

impl super::Node for Children {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        match self {
            Children::Single(node) => node.clone(),
            Children::StaticMultiple(arr) => {
                AnyNode(react_sys::create_fragment(&JsValue::NULL, arr).into())
            }
        }
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        Some(self.clone())
    }

    #[inline]
    fn into_react_node_js(self) -> AnyNode {
        match self {
            Children::Single(node) => node,
            _ => self.as_react_node_js(),
        }
    }

    #[inline]
    fn into_react_children_js(self) -> Option<Children>
    where
        Self: Sized,
    {
        Some(self)
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
