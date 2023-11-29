use async_str_iter::flat::Flat;

use crate::SsrElement;

pub struct IterHtmlChildren<I: Iterator>(I)
where
    I::Item: SsrElement;

impl<I: Iterator> Iterator for IterHtmlChildren<I>
where
    I::Item: SsrElement,
{
    type Item = <I::Item as SsrElement>::HtmlChildren;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(<I::Item as SsrElement>::into_html_children)
    }
}

impl<E: SsrElement, const N: usize> SsrElement for [E; N] {
    type HtmlChildren = Flat<IterHtmlChildren<std::array::IntoIter<E, N>>>;

    fn into_html_children(self) -> Self::HtmlChildren {
        Self::HtmlChildren::new(IterHtmlChildren(self.into_iter()))
    }
}
