pub trait ReceiveNode<V> {
    type Output;
    fn receive_node(self, value: V) -> Self::Output;
}

pub trait StartElement<TBuilding: super::ElementBuilding> {
    type Output;
    fn start_element(self, building_data: TBuilding) -> Self::Output;
}

pub trait EndElement {
    type Building: super::ElementBuilding;
    type Children: super::UnwrapChildrenValue<<Self::Building as super::ElementBuilding>::Builder>;
    type Output;

    /// closure `build_props` should be like
    /// `|builder, children| builder.children(children)`
    fn end_element<
        F: FnOnce(
            <Self::Children as super::UnwrapChildrenValue<<Self::Building as super::ElementBuilding>::Builder>>::PropsBuilderWrapper,
            <Self::Children as super::UnwrapChildrenValue<<Self::Building as super::ElementBuilding>::Builder>>::ChildrenValue,
        ) -> <<Self::Building as super::ElementBuilding>::ComponentType as react::Component>::Props,
    >(
        self,
        build_props: F,
    ) -> Self::Output;
}
