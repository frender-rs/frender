use crate::ToElement;

use super::{MapToElement, MutCsrElementWithValueUsingMapToElement};

#[derive(Debug, Clone, Copy)]
pub struct WithToElement;

impl MutCsrElementWithValueUsingMapToElement for WithToElement {}

impl<V: ?Sized + ToElement> MapToElement<V> for WithToElement {
    type RefToElement<'a> = V::ToElement<'a>
    where
        V: 'a;

    type RefToElementHtmlChildren = V::ToElementHtmlChildren;
    type RefToElementRenderStateKind = V::ToElementRenderStateKind;

    fn map_to_element<'a>(&mut self, v: &'a V) -> Self::RefToElement<'a> {
        v.to_element()
    }
}
