use crate::{FnMutMapRefToElement, FnOnceOutputElement};

use super::{MapToElement, MutCsrElementWithValueUsingMapToElement};

#[derive(Debug, Clone, Copy)]
pub struct WithFn<F>(pub F);

impl<F> MutCsrElementWithValueUsingMapToElement for WithFn<F> {}

impl<V, F> MapToElement<V> for WithFn<F>
where
    V: ?Sized,
    F: FnMutMapRefToElement<V>,
{
    type RefToElement<'a> = <F as FnOnceOutputElement<&'a V>>::OutputElement
    where
        Self: 'a,
        V: 'a;

    type RefToElementHtmlChildren = F::RefToElementHtmlChildren;
    type RefToElementRenderStateKind = F::RefToElementRenderStateKind;

    fn map_to_element<'a>(&'a mut self, v: &'a V) -> Self::RefToElement<'a> {
        (self.0)(v)
    }
}
