use std::cell::RefCell;

use super::AllStates;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;
#[cfg(feature = "ToElement")]
mod with_to_element;

pub trait MapItemToElement<Item> {
    type ItemToElement;
    fn map_item_to_element(&mut self, item: Item) -> Self::ItemToElement;
}

impl<F, Item, E> MapItemToElement<Item> for F
where
    F: FnMut(Item) -> E,
{
    type ItemToElement = E;

    #[inline(always)]
    fn map_item_to_element(&mut self, item: Item) -> Self::ItemToElement {
        self(item)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MapItemWithToElement;

#[cfg_attr(not(any(feature = "csr", feature = "ssr",)), allow(dead_code))] // fields are not used
pub struct SyncedCollectionToElement<
    'a,
    ES: Iterator,
    F: MapItemToElement<ES::Item> = MapItemWithToElement,
> {
    pub(super) all_states: &'a RefCell<AllStates>,
    pub(super) items: ES,
    pub(super) f: F,
}
