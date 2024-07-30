use crate::{FnMutMapRefToElement, FnOnceOutputElement};

use super::{AsMutCsrElementWithValue, IntoHtmlChildrenWithValue, SelfAsMutCsrElementWithValue};

#[derive(Debug, Clone, Copy)]
pub struct WithFn<F>(pub F);

impl<F> SelfAsMutCsrElementWithValue for WithFn<F> {}

impl<V, F> AsMutCsrElementWithValue<V> for WithFn<F>
where
    V: ?Sized,
    F: FnMutMapRefToElement<V>,
{
    type ElementWithValue<'a> = <F as FnOnceOutputElement<&'a V>>::OutputElement
    where
        Self: 'a,
        V: 'a;

    type ElementWithValueRenderStateKind = F::RefToElementRenderStateKind;

    fn as_mut_csr_element_with_value<'a>(&'a mut self, value: &'a V) -> Self::ElementWithValue<'a> {
        (self.0)(value)
    }
}

impl<V, F> IntoHtmlChildrenWithValue<V> for WithFn<F>
where
    V: ?Sized,
    F: FnMutMapRefToElement<V>,
{
    type HtmlChildrenWithValue = F::RefToElementHtmlChildren;

    fn into_html_children_with_value(mut self, value: &V) -> Self::HtmlChildrenWithValue {
        use crate::SsrElement as _;
        (self.0)(value).into_html_children()
    }
}
