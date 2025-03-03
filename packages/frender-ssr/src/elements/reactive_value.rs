use frender_reactive_value::{ReactiveValueIntoElement, ReactiveValueWithKind};

use crate::SsrElement;

/// [`ReactiveValueIntoElement<V>`] just derives ssr.
impl<V: ReactiveValueWithKind + SsrElement> SsrElement for ReactiveValueIntoElement<V> {
    type HtmlChildren = V::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}

impl<V: ReactiveValueWithKind + SsrElement + Copy> super::KnownCopySsrElement
    for ReactiveValueIntoElement<V>
{
}
