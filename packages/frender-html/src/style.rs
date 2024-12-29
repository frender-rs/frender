use std::marker::PhantomData;

use frender_common::convert::FromMut as _;
use frender_dom::behaviors::ElementWithStyle;
use frender_style::csr::{CsrStyle, CssStyleDeclaration};

use crate::{
    html::{behavior_type_traits, prop_markers},
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

pub(crate) trait HasStyleDomApi<BT: BehaviorType> {
    type StyleDomApi<'a, R: 'a + ?Sized + RenderHtml>: CssStyleDeclaration
    where
        BT: 'a;

    fn style_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut BT::OfBehaviorType<R>, renderer: &'a mut R) -> Self::StyleDomApi<'a, R>
    where
        BT: 'a;
}

impl<ET: behavior_type_traits::HtmlElement> HasStyleDomApi<ET> for prop_markers::HtmlElement::style {
    type StyleDomApi<'a, R: 'a + ?Sized + RenderHtml> = <ET::HtmlElement<R> as ElementWithStyle<R>>::Style<'a>
    where
        ET: 'a;

    fn style_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut <ET as BehaviorType>::OfBehaviorType<R>, renderer: &'a mut R) -> Self::StyleDomApi<'a, R>
    where
        ET: 'a,
    {
        <ET::HtmlElement<R>>::from_mut(b).style(renderer)
    }
}

enum Never {}
pub struct Kind<PM, S>(Never, PhantomData<PM>, PhantomData<S>);

impl<PM, S> UnpinnedNonReactiveRenderStateKind for Kind<PM, S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = S;
}

impl<PM: HasStyleDomApi<BT>, V: CsrStyle, BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for Property<PM, V> {
    type UnpinnedRenderStateKind = Kind<PM, V::UpdateWithState>;

    fn unpinned_render_init_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        let mut state = Default::default();
        V::update_with_state(value, &mut state, &mut PM::style_dom_api(b, renderer));
        state
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        V::update_with_state(value, state, &mut PM::style_dom_api(b, renderer));
    }
}

pub(crate) mod impl_bounds {
    pub(crate) use super::{ssr, Property};
    pub(crate) use frender_style::Style as Bounds;
}

pub(crate) mod ssr {
    use frender_ssr::html::attr_value::AttrEqValue;
    use frender_style::ssr::{SsrDeclarationList, SsrStyle};

    use crate::impl_bounds::ssr::SpaceAndHtmlAttributes;

    type Haevoe<V> = AttrEqValue<<<V as SsrStyle>::IntoSsrDeclarationList as SsrDeclarationList>::IntoDeclarationList>;

    pub(crate) fn into_haevoe<V: SsrStyle>(this: V) -> Haevoe<V> {
        Haevoe::<V>::new(SsrDeclarationList::into_declaration_list(V::into_ssr_declaration_list(this)))
    }

    pub(crate) type Output<V> = SpaceAndHtmlAttributes<Haevoe<V>>;

    // Style attribute is always present if specified
    pub(crate) use crate::impl_bounds::ssr::into_space_and_html_attributes as output;
}
