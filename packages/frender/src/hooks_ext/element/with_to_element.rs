use crate::ToElement;

use super::{AsMutCsrElementWithValue, IntoHtmlChildrenWithValue, SelfAsMutCsrElementWithValue};

#[derive(Debug, Clone, Copy)]
pub struct WithToElement;

impl SelfAsMutCsrElementWithValue for WithToElement {}

impl<V: ?Sized + ToElement> IntoHtmlChildrenWithValue<V> for WithToElement {
    type HtmlChildrenWithValue = V::ToElementHtmlChildren;

    fn into_html_children_with_value(self, value: &V) -> Self::HtmlChildrenWithValue {
        use frender_ssr::SsrElement as _;
        value.to_element().into_html_children()
    }
}

impl<V: ?Sized + ToElement> AsMutCsrElementWithValue<V> for WithToElement {
    type ElementWithValue<'a> = V::ToElement<'a>
    where
        V: 'a;

    type ElementWithValueRenderStateKind = V::ToElementRenderStateKind;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, v: &'a V) -> Self::ElementWithValue<'a> {
        v.to_element()
    }
}
