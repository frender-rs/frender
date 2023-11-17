pub use crate::html::behaviors as node_behaviors;

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
}

pub trait CreateNode {
    fn create_node<R: crate::html::RenderHtml>(renderer: &mut R) -> Self::Node<R>
    where
        Self: crate::html::behavior_type_traits::Node;
}
