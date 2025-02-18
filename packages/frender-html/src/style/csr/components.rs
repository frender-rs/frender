use frender_common::convert::FromMut as _;
use frender_dom::behaviors::ElementWithStyle;
use frender_style::csr::{CsrStyle, CssStyleDeclaration};

use crate::{
    html::{behavior_type_traits, prop_markers},
    update_element::{UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    BehaviorType, RenderHtml,
};

use super::HasStyleDomApi;

impl<ET: behavior_type_traits::HtmlElement> HasStyleDomApi<ET> for prop_markers::HtmlElement::style {
    type StyleDomApi<'a, R: 'a + ?Sized + RenderHtml>
        = <ET::HtmlElement<R> as ElementWithStyle<R>>::Style<'a>
    where
        ET: 'a;

    fn style_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut <ET as BehaviorType>::OfBehaviorType<R>, renderer: &'a mut R) -> Self::StyleDomApi<'a, R>
    where
        ET: 'a,
    {
        <ET::HtmlElement<R>>::from_mut(b).style(renderer)
    }
}
