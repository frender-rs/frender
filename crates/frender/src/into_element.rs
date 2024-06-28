use frender_html::RenderStateKind;
use frender_ssr::html::assert::HtmlChildren;

use crate::Element;

pub trait IntoElement {
    fn into_element(self) -> impl Element;
}

impl<E: Element> IntoElement for E {
    fn into_element(self) -> impl Element {
        self
    }
}

/// Trait alias for `IntoIterator<Item: IntoElement>`.
pub trait IntoElements: IntoIterator<Item = Self::ItemImplIntoElement> {
    type ItemImplIntoElement: IntoElement;
}

impl<T> IntoElements for T
where
    T: IntoIterator,
    T::Item: IntoElement,
{
    type ItemImplIntoElement = T::Item;
}
