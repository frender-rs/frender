use frender_ssr::SsrElement;

use super::{MapItemToElement, SyncedCollectionToElement};

impl<'a, ES: Iterator, F: MapItemToElement<ES::Item>> SsrElement
    for SyncedCollectionToElement<'a, ES, F>
where
    F::ItemToElement: SsrElement,
{
    type HtmlChildren = async_str_iter::flat::Flat<
        std::vec::IntoIter<<F::ItemToElement as SsrElement>::HtmlChildren>,
    >;

    fn into_html_children(mut self) -> Self::HtmlChildren {
        let _ = self.all_states; // use this field to avoid warnings
        let children = self
            .items
            .map(|el| self.f.map_item_to_element(el).into_html_children())
            .collect::<Vec<_>>();
        async_str_iter::flat::Flat::new(children.into_iter())
    }
}
