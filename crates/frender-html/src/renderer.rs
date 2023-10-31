pub use crate::html::behaviors as node_behaviors;

pub trait HasIntrinsicComponentTag {
    const INTRINSIC_COMPONENT_TAG: &'static str;
}

pub trait CreateNode {
    fn create_node<R: crate::html::RenderHtml>(renderer: &mut R) -> Self::Node<R>
    where
        Self: crate::html::behavior_type_traits::Node;
}

pub trait RenderTextFrom<Text, V: ?Sized> {
    /// should not move cursor
    fn render_text_from(&mut self, v: &V) -> Text;
    fn update_text_from(&mut self, text: &mut Text, v: &V);
}
