use std::marker::PhantomData;

use frender_common::convert::FromMut as _;
use frender_dom::csr::behaviors::ElementWithStyle;
use frender_style::csr::{CsrStyle, CssStyleDeclaration};

use crate::{
    html::behavior_type_traits,
    update_element::{UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    BehaviorType, RenderHtml,
};

use super::Property;

#[cfg(feature = "components")]
mod components;

pub(crate) trait HasStyleDomApi<BT: BehaviorType> {
    type StyleDomApi<'a, R: 'a + ?Sized + RenderHtml>: CssStyleDeclaration
    where
        BT: 'a;

    fn style_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut BT::OfBehaviorType<R>, renderer: &'a mut R) -> Self::StyleDomApi<'a, R>
    where
        BT: 'a;
}

enum Never {}
pub struct Kind<PM, S>(Never, PhantomData<PM>, PhantomData<S>);

impl<PM, S> UnpinnedNonReactiveRenderStateKind for Kind<PM, S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = S;
}

impl<PM: HasStyleDomApi<BT>, V: CsrStyle, BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for Property<PM, V> {
    type UnpinnedRenderStateKind = Kind<PM, V::State>;

    fn unpinned_render_init_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        V::csr_style_render_init(value, &mut PM::style_dom_api(b, renderer))
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + crate::RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as crate::BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as crate::update_element::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        V::csr_style_render_update(value, &mut PM::style_dom_api(b, renderer), state)
    }
}
