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

pub trait ToElement {
    type ToElementHtmlChildren: HtmlChildren;
    type ToElementRenderStateKind: RenderStateKind;
    type ToElement<'a>: Element<
        HtmlChildren = Self::ToElementHtmlChildren,
        RenderStateKind = Self::ToElementRenderStateKind,
    >
    where
        Self: 'a;
    fn to_element(&self) -> Self::ToElement<'_>;
}

#[derive(Debug, Clone, Copy)]
pub struct ToElementWithFn<E, F>(pub E, pub F);

mod sealed {
    use crate::Element;

    pub trait FnOutputElement<Arg>: Fn(Arg) -> Self::OutputElement {
        type OutputElement: Element;
    }

    impl<F: Fn(Arg) -> E, Arg, E: Element> FnOutputElement<Arg> for F {
        type OutputElement = E;
    }
}

impl<E, F: for<'a> sealed::FnOutputElement<&'a E>, C, K> ToElement for ToElementWithFn<E, F>
where
    for<'a> <F as sealed::FnOutputElement<&'a E>>::OutputElement:
        Element<HtmlChildren = C, RenderStateKind = K>,
    C: HtmlChildren,
    K: RenderStateKind,
{
    type ToElement<'a> = <F as sealed::FnOutputElement<&'a E>>::OutputElement
    where
        Self: 'a;

    type ToElementHtmlChildren = C;

    type ToElementRenderStateKind = K;

    fn to_element(&self) -> Self::ToElement<'_> {
        (self.1)(&self.0)
    }
}
