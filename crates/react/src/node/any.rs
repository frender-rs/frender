use std::rc::Rc;

use wasm_bindgen::JsValue;

use super::{AnyNodeValue, Node};

#[derive(Debug, Clone)]
pub enum AnyNode {
    /// No node.
    /// Will be converted to `null` when passed to js
    Null,
    /// Valid `ReactNode` excluding elements.
    /// Such as number, string, boolean.
    Value(AnyNodeValue),
    /// Element
    Element(crate::Element),
    /// Array of keyed elements
    Multiple(Rc<Vec<crate::Keyed<crate::Element>>>),
}

impl AnyNode {
    #[inline]
    pub fn null() -> Self {
        Self::Null
    }

    /// Consumes `AnyNode` and output the node value
    /// so that it can be passed to js.
    ///
    /// Note that the produced JsValue shouldn't
    /// be cloned due to the following reasons:
    ///
    /// If it is an element
    /// it will drop the `use_render` closure
    /// which has been previously forgotten in
    /// [`crate::BridgeElementData::unsafe_create_element_js`].
    #[inline]
    pub(crate) fn unsafe_into_js_node_value(self) -> JsValue {
        let js = match self {
            AnyNode::Null => JsValue::NULL,
            AnyNode::Value(v) => v.into(),
            AnyNode::Element(el) => el.unsafe_into_js_element().into(),
            AnyNode::Multiple(els) => (match Rc::try_unwrap(els) {
                Ok(els) => js_sys::Array::from_iter(
                    els.into_iter()
                        .map(|node| node.0.into_node().unsafe_into_js_node_value()),
                ),
                Err(els) => js_sys::Array::from_iter(
                    els.iter()
                        .map(|node| node.0.to_node().unsafe_into_js_node_value()),
                ),
            })
            .into(),
        };
        js
    }
}

impl Node for AnyNode {
    #[inline]
    fn to_node(&self) -> AnyNode {
        self.clone()
    }

    #[inline]
    fn to_children(&self) -> Option<crate::Children> {
        Some(crate::Children::from_single(self.clone()))
    }

    #[inline]
    fn into_node(self) -> AnyNode {
        self
    }

    #[inline]
    fn into_children(self) -> Option<crate::Children>
    where
        Self: Sized,
    {
        Some(crate::Children::from_single(self))
    }
}
