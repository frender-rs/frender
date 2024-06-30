use frender_html::RenderStateKind;
use frender_ssr::html::assert::HtmlChildren;

use crate::Element;

pub trait FnOutputElement<Arg>: Fn(Arg) -> Self::OutputElement {
    type OutputElement: Element<
        HtmlChildren = Self::OutputElementHtmlChildren,
        RenderStateKind = Self::OutputElementRenderStateKind,
    >;
    type OutputElementHtmlChildren: HtmlChildren;
    type OutputElementRenderStateKind: RenderStateKind;
}

impl<F: ?Sized + Fn(Arg) -> E, Arg, E: Element> FnOutputElement<Arg> for F {
    type OutputElement = E;
    type OutputElementHtmlChildren = E::HtmlChildren;
    type OutputElementRenderStateKind = E::RenderStateKind;
}

pub trait FnMapRefToElement<E: ?Sized>:
    for<'a> FnOutputElement<
    &'a E,
    OutputElementHtmlChildren = Self::RefToElementHtmlChildren,
    OutputElementRenderStateKind = Self::RefToElementRenderStateKind,
>
{
    type RefToElementHtmlChildren: HtmlChildren;
    type RefToElementRenderStateKind: RenderStateKind;
}

impl<F, E: ?Sized, C, K> FnMapRefToElement<E> for F
where
    F: for<'a> FnOutputElement<
        &'a E,
        OutputElementHtmlChildren = C,
        OutputElementRenderStateKind = K,
    >,
    C: HtmlChildren,
    K: RenderStateKind,
{
    type RefToElementHtmlChildren = C;
    type RefToElementRenderStateKind = K;
}

pub trait FnMutOutputElement<Arg>: FnMut(Arg) -> Self::OutputElement {
    type OutputElement: Element<
        HtmlChildren = Self::OutputElementHtmlChildren,
        RenderStateKind = Self::OutputElementRenderStateKind,
    >;
    type OutputElementHtmlChildren: HtmlChildren;
    type OutputElementRenderStateKind: RenderStateKind;
}

impl<F: ?Sized + FnMut(Arg) -> E, Arg, E: Element> FnMutOutputElement<Arg> for F {
    type OutputElement = E;
    type OutputElementHtmlChildren = E::HtmlChildren;
    type OutputElementRenderStateKind = E::RenderStateKind;
}

pub trait FnMutMapRefToElement<E: ?Sized>:
    for<'a> FnMutOutputElement<
    &'a E,
    OutputElementHtmlChildren = Self::RefToElementHtmlChildren,
    OutputElementRenderStateKind = Self::RefToElementRenderStateKind,
>
{
    type RefToElementHtmlChildren: HtmlChildren;
    type RefToElementRenderStateKind: RenderStateKind;
}

impl<F, E: ?Sized, C, K> FnMutMapRefToElement<E> for F
where
    F: for<'a> FnMutOutputElement<
        &'a E,
        OutputElementHtmlChildren = C,
        OutputElementRenderStateKind = K,
    >,
    C: HtmlChildren,
    K: RenderStateKind,
{
    type RefToElementHtmlChildren = C;
    type RefToElementRenderStateKind = K;
}
