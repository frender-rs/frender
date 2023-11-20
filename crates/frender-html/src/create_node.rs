pub trait CreateNode {
    fn create_node<R: crate::html::RenderHtml>(renderer: &mut R) -> Self::Node<R>
    where
        Self: crate::html::behavior_type_traits::Node;
}