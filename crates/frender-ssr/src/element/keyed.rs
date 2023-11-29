use frender_common::{Elements, Keyed};

use crate::SsrElement;

impl<K, E> SsrElement for Vec<Keyed<K, E>>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: SsrElement,
{
    type HtmlChildren = async_str_iter::flat::Flat<IterKeyed<std::vec::IntoIter<Keyed<K, E>>>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::flat::Flat::new(IterKeyed(self.into_iter()))
    }
}

pub struct IterKeyed<I>(I);

impl<K, E, I: Iterator<Item = Keyed<K, E>>> Iterator for IterKeyed<I>
where
    E: SsrElement,
{
    type Item = E::HtmlChildren;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|Keyed(_, e)| e.into_html_children())
    }
}

impl<K, E, A, I: IntoIterator<Item = Keyed<K, E>>> SsrElement for Elements<I, A>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: SsrElement,
{
    type HtmlChildren = async_str_iter::flat::Flat<IterKeyed<I::IntoIter>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::flat::Flat::new(IterKeyed(self.iter.into_iter()))
    }
}
