use frender_html::RenderStateKind;
use frender_ssr::html::assert::HtmlChildren;

use crate::Element;

pub trait FnOnceOutputElement<Arg>: FnOnce(Arg) -> Self::OutputElement {
    type OutputElement: Element<
        HtmlChildren = Self::OutputElementHtmlChildren,
        RenderStateKind = Self::OutputElementRenderStateKind,
    >;
    type OutputElementHtmlChildren: HtmlChildren;
    type OutputElementRenderStateKind: RenderStateKind;
}

impl<Arg, F: ?Sized + FnOnce(Arg) -> E, E: Element> FnOnceOutputElement<Arg> for F {
    type OutputElement = E;
    type OutputElementHtmlChildren = E::HtmlChildren;
    type OutputElementRenderStateKind = E::RenderStateKind;
}

pub trait FnMutOutputElement<Arg>:
    FnOnceOutputElement<Arg> + FnMut(Arg) -> Self::OutputElement
{
}

impl<Arg, F: ?Sized + FnMut(Arg) -> E, E: Element> FnMutOutputElement<Arg> for F {}

pub trait FnOutputElement<Arg>: FnMutOutputElement<Arg> + Fn(Arg) -> Self::OutputElement {}

impl<Arg, F: ?Sized + Fn(Arg) -> E, E: Element> FnOutputElement<Arg> for F {}

pub trait FnOnce2OutputElement<A1, A2>: FnOnce(A1, A2) -> Self::OutputElement {
    type OutputElement: Element<
        HtmlChildren = Self::OutputElementHtmlChildren,
        RenderStateKind = Self::OutputElementRenderStateKind,
    >;
    type OutputElementHtmlChildren: HtmlChildren;
    type OutputElementRenderStateKind: RenderStateKind;
}

impl<A1, A2, F: ?Sized + FnOnce(A1, A2) -> E, E: Element> FnOnce2OutputElement<A1, A2> for F {
    type OutputElement = E;
    type OutputElementHtmlChildren = E::HtmlChildren;
    type OutputElementRenderStateKind = E::RenderStateKind;
}

pub trait FnMut2OutputElement<A1, A2>:
    FnOnce2OutputElement<A1, A2> + FnMut(A1, A2) -> Self::OutputElement
{
}

impl<A1, A2, F: ?Sized + FnMut(A1, A2) -> E, E: Element> FnMut2OutputElement<A1, A2> for F {}

pub trait Fn2OutputElement<A1, A2>:
    FnMut2OutputElement<A1, A2> + Fn(A1, A2) -> Self::OutputElement
{
}

impl<A1, A2, F: ?Sized + Fn(A1, A2) -> E, E: Element> Fn2OutputElement<A1, A2> for F {}

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

pub trait FnMutMap2RefsToElement<A1: ?Sized, A2: ?Sized>:
    for<'a, 'b> FnMut2OutputElement<
    &'a A1,
    &'b A2,
    OutputElementHtmlChildren = Self::Refs2ToElementHtmlChildren,
    OutputElementRenderStateKind = Self::Refs2ToElementRenderStateKind,
>
{
    type Refs2ToElementHtmlChildren: HtmlChildren;
    type Refs2ToElementRenderStateKind: RenderStateKind;
}

impl<F, A1: ?Sized, A2: ?Sized, C, K> FnMutMap2RefsToElement<A1, A2> for F
where
    F: for<'a, 'b> FnMut2OutputElement<
        &'a A1,
        &'b A2,
        OutputElementHtmlChildren = C,
        OutputElementRenderStateKind = K,
    >,
    C: HtmlChildren,
    K: RenderStateKind,
{
    type Refs2ToElementHtmlChildren = C;
    type Refs2ToElementRenderStateKind = K;
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
