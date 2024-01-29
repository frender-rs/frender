use crate::BehaviorType;

pub trait CreateNode {
    fn create_node<R: crate::html::RenderHtml + ?Sized>(renderer: &mut R) -> Self::NodeOfBehaviorType<R>
    where
        Self: BehaviorType;
}
