use std::marker::PhantomData;

use frender_common::convert::FromMut as _;
use frender_dom::{
    behaviors::{ElementWithClassList, ElementWithRelList},
    dom_tokens::{DomTokenList, DomTokens},
};

use crate::{
    html::{behavior_type_traits, prop_markers},
    intrinsic::AttributeState,
    update_element::{UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    BehaviorType, RenderHtml,
};

pub struct Property<PM, V> {
    _prop_marker: PhantomData<PM>,
    value: V,
}

impl<PM, V> Property<PM, V> {
    pub(crate) fn new(value: V) -> Self {
        Self { _prop_marker: PhantomData, value }
    }
}

pub(crate) trait HasDomTokensDomApi<BT: BehaviorType> {
    type DomTokensDomApi<'a, R: 'a + ?Sized + RenderHtml>: DomTokenList
    where
        BT: 'a;

    fn dom_tokens_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut BT::OfBehaviorType<R>, renderer: &'a mut R) -> Self::DomTokensDomApi<'a, R>
    where
        BT: 'a;
}

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

enum Never {}
pub struct Kind<PM, S>(Never, PhantomData<PM>, PhantomData<S>);

impl<PM, S> UnpinnedNonReactiveRenderStateKind for Kind<PM, S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = AttributeState<PhantomData<PM>, S>;
}

impl<PM: HasDomTokensDomApi<BT>, V: DomTokens, BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for Property<PM, V> {
    type UnpinnedRenderStateKind = Kind<PM, V::State>;

    fn unpinned_render_init_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        AttributeState(_prop_marker, V::dom_tokens_render_init(value, &mut PM::dom_tokens_dom_api(b, renderer)))
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
        AttributeState(PhantomData, state): &mut <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        V::dom_tokens_render_update(value, &mut PM::dom_tokens_dom_api(b, renderer), state)
    }
}

pub(crate) mod impl_bounds {
    pub(crate) use super::{ssr, Property};
    pub(crate) use frender_dom::dom_tokens::DomTokens as Bounds;
}

pub(crate) mod ssr {
    use frender_dom::dom_tokens::DomTokens;
    use frender_ssr::html::attr_value::AttrEqValue;

    use crate::impl_bounds::ssr::SpaceAndHtmlAttributes;

    type Haevoe<V> = AttrEqValue<<V as DomTokens>::DomTokensIntoAsyncStrIter>;

    pub(crate) fn into_haevoe<V: DomTokens>(this: V) -> Haevoe<V> {
        Haevoe::<V>::new(V::dom_tokens_into_async_str_iter(this))
    }

    pub(crate) type Output<V> = SpaceAndHtmlAttributes<Haevoe<V>>;

    // DomTokens attributes are always present if specified
    pub(crate) use crate::impl_bounds::ssr::into_space_and_html_attributes as output;
}
