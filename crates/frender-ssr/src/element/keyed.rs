use frender_common::{Elements, Keyed};

use crate::Element;

impl<K, E> Element for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: Element,
{
    type HtmlChildren = crate::async_str::flat::Flat<IterKeyed<std::vec::IntoIter<Keyed<K, E>>>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        crate::async_str::flat::Flat::new(IterKeyed(self.into_iter()))
    }
}

pub struct IterKeyed<I>(I);

impl<K, E, I: Iterator<Item = Keyed<K, E>>> Iterator for IterKeyed<I>
where
    E: Element,
{
    type Item = E::HtmlChildren;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|Keyed(_, e)| e.into_html_children())
    }
}

// impl<K, E> IntoAsyncStrIterator for crate::Keyed<K, E>
// where
//     K: std::hash::Hash + Eq, // TODO: ToString ?
//     E: Element,
// {
//     type IntoAsyncStrIterator = E::HtmlElements;

//     fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
//         self.1.into_html_elements()
//     }
// }

impl<K, E, A, I: IntoIterator<Item = Keyed<K, E>>> Element for Elements<I, A>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: Element,
{
    type HtmlChildren = crate::async_str::flat::Flat<IterKeyed<I::IntoIter>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        crate::async_str::flat::Flat::new(IterKeyed(self.iter.into_iter()))
    }
}
