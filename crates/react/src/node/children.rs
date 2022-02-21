use std::rc::Rc;

use crate::{AnyNode, ComponentStatic};

/// One or many [`AnyNode`]s whose order will
/// never change.
/// When `Children` is used as
#[derive(Debug, Clone)]
pub enum Children {
    Single(Box<AnyNode>),
    StaticMultiple(Rc<Vec<AnyNode>>),
}

impl Children {
    #[inline]
    pub fn from_static_nodes<T: IntoIterator<Item = AnyNode>>(nodes: T) -> Self {
        Self::StaticMultiple(Rc::new(Vec::from_iter(nodes)))
    }

    pub fn from_single(node: AnyNode) -> Children {
        Self::Single(Box::new(node))
    }

    /// See [`AnyNode::unsafe_into_js_node_value`] for the safety notes.
    #[inline]
    pub(crate) fn unsafe_into_js_array(self) -> js_sys::Array {
        match self {
            Children::Single(node) => js_sys::Array::of1(&node.unsafe_into_js_node_value()),
            Children::StaticMultiple(arr) => match Rc::try_unwrap(arr) {
                Ok(arr) => js_sys::Array::from_iter(
                    arr.into_iter().map(AnyNode::unsafe_into_js_node_value),
                ),
                Err(arr) => js_sys::Array::from_iter(
                    arr.iter()
                        .map(|v| AnyNode::unsafe_into_js_node_value(v.clone())),
                ),
            },
        }
    }
}

impl super::Node for Children {
    #[inline]
    fn as_react_node_js(&self) -> AnyNode {
        self.clone().into_react_node_js()
    }

    #[inline]
    fn as_react_children_js(&self) -> Option<Children> {
        Some(self.clone())
    }

    /// Returns the single node or wrap multiple nodes in a Fragment
    #[inline]
    fn into_react_node_js(self) -> AnyNode {
        match self {
            Children::Single(node) => *node,
            children => AnyNode::Element(
                crate::Fragment::create_element(
                    crate::OptionalChildrenProps {
                        children: Some(children),
                    },
                    None,
                )
                .into(),
            ),
        }
    }

    #[inline]
    fn into_react_children_js(self) -> Option<Children> {
        Some(self)
    }
}
