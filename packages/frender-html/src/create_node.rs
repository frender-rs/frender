use crate::BehaviorType;

pub trait CreateNode {
    fn create_node<R: crate::html::RenderHtml + ?Sized>(renderer: &mut R) -> Self::OfBehaviorType<R>
    where
        Self: BehaviorType;
}
