use react::Component;

use super::super::{
    ElementBuilding, EndElement, ReceiveNode, RsxChildren, StartElement, UnwrapChildrenValue,
};

pub struct ExpectNodes<TReceiver, TBuilding: ElementBuilding, TChildren> {
    /// When calling `end_node`, the receiver will receive the built element.
    receiver: TReceiver,
    /// Current parent node that might have children.
    building: TBuilding,
    /// The current children which might receive more nodes
    children: TChildren,
}

impl<TReceiver, TBuilding: ElementBuilding, TChildren>
    ExpectNodes<TReceiver, TBuilding, TChildren>
{
    pub fn new(receiver: TReceiver, building: TBuilding, children: TChildren) -> Self {
        Self {
            receiver,
            building,
            children,
        }
    }
}

/// If the children can receive new child,
/// then `ExpectNodes` can receive new child.
impl<TNewChild, TReceiver, TBuilding: ElementBuilding, TChildren: ReceiveNode<TNewChild>>
    ReceiveNode<TNewChild> for ExpectNodes<TReceiver, TBuilding, TChildren>
{
    type Output = ExpectNodes<TReceiver, TBuilding, TChildren::Output>;

    fn receive_node(self, value: TNewChild) -> Self::Output {
        ExpectNodes {
            receiver: self.receiver,
            building: self.building,
            children: self.children.receive_node(value),
        }
    }
}

impl<TNewBuilding: ElementBuilding, TReceiver, TBuilding: ElementBuilding, TChildren>
    StartElement<TNewBuilding> for ExpectNodes<TReceiver, TBuilding, TChildren>
{
    type Output = ExpectNodes<Self, TNewBuilding, RsxChildren<()>>;

    #[inline]
    fn start_element(self, building_data: TNewBuilding) -> Self::Output {
        ExpectNodes {
            receiver: self,
            building: building_data,
            children: RsxChildren(()),
        }
    }
}

impl<TReceiver, TBuilding: ElementBuilding, TChildren> EndElement
    for ExpectNodes<TReceiver, TBuilding, TChildren>
where
    TReceiver:
        ReceiveNode<<<TBuilding as ElementBuilding>::ComponentType as Component>::ElementType>,
    TChildren: UnwrapChildrenValue<TBuilding::Builder>,
{
    type Building = TBuilding;
    type Children = TChildren;
    type Output = TReceiver::Output;

    fn end_element<
        F: FnOnce(
            <TChildren as UnwrapChildrenValue<TBuilding::Builder>>::PropsBuilderWrapper,
            <TChildren as UnwrapChildrenValue<TBuilding::Builder>>::ChildrenValue,
        ) -> <TBuilding::ComponentType as Component>::Props,
    >(
        self,
        build_props: F,
    ) -> Self::Output {
        let building_data = self.building.unwrap_building_data();
        let builder_wrapper = TChildren::wrap_props_builder(building_data.props_builder);
        let children_value = self.children.unwrap_children_value();
        let props = build_props(builder_wrapper, children_value);

        let el =
            TBuilding::ComponentType::new_with_props(props).call_create_element(building_data.key);

        self.receiver.receive_node(el)
    }
}
