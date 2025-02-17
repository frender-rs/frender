use crate::ToElement;

use super::{
    super::SyncedCollection, MapItemToElement, MapItemWithToElement, SyncedCollectionToElement,
};

impl<'a, Item: ToElement> MapItemToElement<&'a Item> for MapItemWithToElement {
    type ItemToElement = Item::ToElement<'a>;
    fn map_item_to_element(&mut self, item: &'a Item) -> Self::ItemToElement {
        item.to_element()
    }
}

impl<ES, E: ToElement> ToElement for SyncedCollection<ES>
where
    for<'a> &'a ES: IntoIterator<Item = &'a E>,
    // TODO: make this implied in RenderStateKind, or make RenderState and UnpinnedRenderState 'static
    // E::ToElementRenderStateKind: 'static,
{
    type ToElement<'a>
        = SyncedCollectionToElement<'a, <&'a ES as IntoIterator>::IntoIter, MapItemWithToElement>
    where
        Self: 'a;

    fn to_element(&self) -> Self::ToElement<'_> {
        self.to_element_with(MapItemWithToElement)
    }
}
