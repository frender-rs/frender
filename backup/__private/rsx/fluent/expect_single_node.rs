use super::super::{ElementBuilding, ReceiveNode, RsxChildren, StartElement};

pub struct ExpectSingleNode;

impl<TBuilding: ElementBuilding> StartElement<TBuilding> for ExpectSingleNode {
    type Output = super::ExpectNodes<Self, TBuilding, RsxChildren<()>>;

    #[inline]
    fn start_element(self, building_data: TBuilding) -> Self::Output {
        super::ExpectNodes::new(self, building_data, RsxChildren(()))
    }
}

impl<V> ReceiveNode<V> for ExpectSingleNode {
    type Output = V;

    #[inline]
    fn receive_node(self, value: V) -> Self::Output {
        value
    }
}
