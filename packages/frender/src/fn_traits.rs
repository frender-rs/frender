use frender_html::{
    csr::experimental::{PinnedRenderStateKind, UnpinnedRenderStateKind},
    csr::{CsrElement, RenderStateKind},
};
use frender_ssr::{html::assert::HtmlChildren, SsrElement};

use crate::Element;

pub trait FnOnce1<Arg>: FnOnce(Arg) -> Self::Output_ {
    type Output_;
}

impl<F: ?Sized + FnOnce(Arg) -> Out, Arg, Out> FnOnce1<Arg> for F {
    type Output_ = Out;
}

pub trait FnMut1<Arg>: FnOnce1<Arg> + FnMut(Arg) -> Self::Output_ {}
impl<F: ?Sized + FnMut(Arg) -> Out, Arg, Out> FnMut1<Arg> for F {}

pub trait Fn1<Arg>: FnMut1<Arg> + Fn(Arg) -> Self::Output_ {}
impl<F: ?Sized + Fn(Arg) -> Out, Arg, Out> Fn1<Arg> for F {}

pub trait FnOnce2<Arg0, Arg1>: FnOnce(Arg0, Arg1) -> Self::Output_ {
    type Output_;
}

impl<F: ?Sized + FnOnce(Arg0, Arg1) -> Out, Arg0, Arg1, Out> FnOnce2<Arg0, Arg1> for F {
    type Output_ = Out;
}

pub trait FnMut2<Arg0, Arg1>: FnOnce2<Arg0, Arg1> + FnMut(Arg0, Arg1) -> Self::Output_ {}
impl<F: ?Sized + FnMut(Arg0, Arg1) -> Out, Arg0, Arg1, Out> FnMut2<Arg0, Arg1> for F {}

pub trait Fn2<Arg0, Arg1>: FnMut2<Arg0, Arg1> + Fn(Arg0, Arg1) -> Self::Output_ {}
impl<F: ?Sized + Fn(Arg0, Arg1) -> Out, Arg0, Arg1, Out> Fn2<Arg0, Arg1> for F {}

pub trait FnOnceOutputSsrElement<Arg>: FnOnce(Arg) -> Self::OutputSsrElement {
    type OutputSsrElement: SsrElement<HtmlChildren = Self::OutputElementHtmlChildren>;
    type OutputElementHtmlChildren: HtmlChildren;
}

impl<Arg, F: ?Sized + FnOnce(Arg) -> Out, Out: SsrElement> FnOnceOutputSsrElement<Arg> for F {
    type OutputSsrElement = Out;
    type OutputElementHtmlChildren = Out::HtmlChildren;
}

pub trait FnOnceOutputCsrElement<Arg>: FnOnce(Arg) -> Self::OutputCsrElement {
    type OutputCsrElement: CsrElement<RenderStateKind = Self::OutputElementRenderStateKind>;
    type OutputElementRenderStateKind: PinnedRenderStateKind + UnpinnedRenderStateKind;
}

impl<Arg, F: ?Sized + FnOnce(Arg) -> Out, Out: CsrElement> FnOnceOutputCsrElement<Arg> for F {
    type OutputCsrElement = Out;
    type OutputElementRenderStateKind = Out::RenderStateKind;
}

pub trait FnOnceOutputElement<Arg>:
    FnOnce(Arg) -> Self::OutputElement
    + FnOnceOutputSsrElement<Arg, OutputSsrElement = Self::OutputElement>
    + FnOnceOutputCsrElement<Arg, OutputCsrElement = Self::OutputElement>
{
    type OutputElement: Element<
        RenderStateKind = Self::OutputElementRenderStateKind,
        HtmlChildren = Self::OutputElementHtmlChildren,
    >;
}

impl<Arg, F: ?Sized + FnOnce(Arg) -> E, E: Element> FnOnceOutputElement<Arg> for F {
    type OutputElement = E;
}

pub trait FnMutOutputElement<Arg>:
    FnOnceOutputElement<Arg> + FnMut(Arg) -> Self::OutputElement
{
}

impl<Arg, F: ?Sized + FnMut(Arg) -> E, E: Element> FnMutOutputElement<Arg> for F {}

pub trait FnOutputElement<Arg>: FnMutOutputElement<Arg> + Fn(Arg) -> Self::OutputElement {}

impl<Arg, F: ?Sized + Fn(Arg) -> E, E: Element> FnOutputElement<Arg> for F {}

pub trait FnOnce2OutputSsrElement<A1, A2>: FnOnce(A1, A2) -> Self::OutputSsrElement {
    type OutputSsrElement: SsrElement<HtmlChildren = Self::OutputElementHtmlChildren>;
    type OutputElementHtmlChildren: HtmlChildren;
}

impl<A1, A2, F: ?Sized + FnOnce(A1, A2) -> Out, Out: SsrElement> FnOnce2OutputSsrElement<A1, A2>
    for F
{
    type OutputSsrElement = Out;
    type OutputElementHtmlChildren = Out::HtmlChildren;
}

pub trait FnOnce2OutputCsrElement<A1, A2>: FnOnce(A1, A2) -> Self::OutputCsrElement {
    type OutputCsrElement: CsrElement<RenderStateKind = Self::OutputElementRenderStateKind>;
    type OutputElementRenderStateKind: PinnedRenderStateKind + UnpinnedRenderStateKind;
}

impl<A1, A2, F: ?Sized + FnOnce(A1, A2) -> Out, Out: CsrElement> FnOnce2OutputCsrElement<A1, A2>
    for F
{
    type OutputCsrElement = Out;
    type OutputElementRenderStateKind = Out::RenderStateKind;
}

pub trait FnOnce2OutputElement<A1, A2>:
    FnOnce(A1, A2) -> Self::OutputElement
    + FnOnce2OutputSsrElement<A1, A2, OutputSsrElement = Self::OutputElement>
    + FnOnce2OutputCsrElement<A1, A2, OutputCsrElement = Self::OutputElement>
{
    type OutputElement: Element<
        RenderStateKind = Self::OutputElementRenderStateKind,
        HtmlChildren = Self::OutputElementHtmlChildren,
    >;
}

impl<A1, A2, F: ?Sized + FnOnce(A1, A2) -> E, E: Element> FnOnce2OutputElement<A1, A2> for F {
    type OutputElement = E;
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
    type Refs2ToElementRenderStateKind: PinnedRenderStateKind + UnpinnedRenderStateKind;
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
    K: PinnedRenderStateKind + UnpinnedRenderStateKind,
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
    type RefToElementRenderStateKind: PinnedRenderStateKind + UnpinnedRenderStateKind;
}

impl<F, E: ?Sized, C, K> FnMutMapRefToElement<E> for F
where
    F: for<'a> FnMutOutputElement<
        &'a E,
        OutputElementHtmlChildren = C,
        OutputElementRenderStateKind = K,
    >,
    C: HtmlChildren,
    K: PinnedRenderStateKind + UnpinnedRenderStateKind,
{
    type RefToElementHtmlChildren = C;
    type RefToElementRenderStateKind = K;
}
