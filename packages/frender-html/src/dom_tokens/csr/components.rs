use frender_common::convert::FromMut as _;
use frender_dom::behaviors::{ElementWithClassList, ElementWithRelList};

use crate::{
    html::{behavior_type_traits, prop_markers},
    BehaviorType, RenderHtml,
};

use super::HasDomTokensDomApi;

impl<ET: behavior_type_traits::Element> HasDomTokensDomApi<ET> for prop_markers::Element::class {
    type DomTokensDomApi<'a, R: 'a + ?Sized + RenderHtml>
        = <ET::Element<R> as ElementWithClassList<R>>::ClassList<'a>
    where
        ET: 'a;

    fn dom_tokens_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut <ET as BehaviorType>::OfBehaviorType<R>, renderer: &'a mut R) -> Self::DomTokensDomApi<'a, R>
    where
        ET: 'a,
    {
        <ET::Element<R>>::from_mut(b).class_list(renderer)
    }
}

impl<ET: behavior_type_traits::ElementWithRelAttribute> HasDomTokensDomApi<ET> for prop_markers::ElementWithRelAttribute::rel {
    type DomTokensDomApi<'a, R: 'a + ?Sized + RenderHtml>
        = <ET::ElementWithRelAttribute<R> as ElementWithRelList<R>>::RelList<'a>
    where
        ET: 'a;

    fn dom_tokens_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut <ET as BehaviorType>::OfBehaviorType<R>, renderer: &'a mut R) -> Self::DomTokensDomApi<'a, R>
    where
        ET: 'a,
    {
        <ET::ElementWithRelAttribute<R>>::from_mut(b).rel_list(renderer)
    }
}
