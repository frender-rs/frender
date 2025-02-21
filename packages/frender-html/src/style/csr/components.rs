use frender_common::convert::FromMut as _;
use frender_dom::csr::behaviors::ElementWithStyle;

use crate::{
    csr::behavior_type::BehaviorType,
    html::{behavior_type_traits, prop_markers, RenderHtml},
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
