use frender_reactive_value::{
    non_reactive::Uncached, ReactiveValueIntoElement, ReactiveValueWithKind,
};

use crate::SsrElement;

/// [`ReactiveValueIntoElement<V>`] just derives ssr.
impl<V: ReactiveValueWithKind + SsrElement> SsrElement for ReactiveValueIntoElement<V> {
    type HtmlChildren = V::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}

/// [`Uncached<V>`] just derives ssr.
impl<V: SsrElement> SsrElement for Uncached<V> {
    type HtmlChildren = V::HtmlChildren;

    fn into_html_children(self) -> Self::HtmlChildren {
        self.0.into_html_children()
    }
}
