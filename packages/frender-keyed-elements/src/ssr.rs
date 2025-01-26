use frender_ssr::SsrElement;

use crate::{Keyed, KeyedElements};

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

impl<K, E, A, I: IntoIterator<Item = Keyed<K, E>>> SsrElement for KeyedElements<I, A>
where
    K: std::hash::Hash + Eq, // TODO: ToString ?
    E: SsrElement,
{
    type HtmlChildren = async_str_iter::flat::Flat<IterKeyed<I::IntoIter>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        async_str_iter::flat::Flat::new(IterKeyed(self.iter.into_iter()))
    }
}
