use std::marker::PhantomData;

use frender_dom::dom_tokens::{DomTokenList, DomTokens};

use crate::{
    csr::behavior_type::{BehaviorType, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    html::RenderHtml,
    intrinsic::AttributeState,
};

use super::Property;

#[cfg(feature = "components")]
mod components;

pub(crate) trait HasDomTokensDomApi<BT: BehaviorType> {
    type DomTokensDomApi<'a, R: 'a + ?Sized + RenderHtml>: DomTokenList
    where
        BT: 'a;

    fn dom_tokens_dom_api<'a, R: ?Sized + RenderHtml>(b: &'a mut BT::OfBehaviorType<R>, renderer: &'a mut R) -> Self::DomTokensDomApi<'a, R>
    where
        BT: 'a;
}

enum Never {}
pub struct Kind<PM, S>(Never, PhantomData<PM>, PhantomData<S>);

impl<PM, S> UnpinnedNonReactiveRenderStateKind for Kind<PM, S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = AttributeState<PhantomData<PM>, S>;
}

impl<PM: HasDomTokensDomApi<BT>, V: DomTokens, BT: BehaviorType> UnpinnedRenderWithBehavior<BT> for Property<PM, V> {
    type UnpinnedRenderStateKind = Kind<PM, V::State>;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        AttributeState(_prop_marker, V::dom_tokens_render_init(value, &mut PM::dom_tokens_dom_api(b, renderer)))
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        Self { _prop_marker, value }: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        AttributeState(PhantomData, state): &mut <Self::UnpinnedRenderStateKind as UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        V::dom_tokens_render_update(value, &mut PM::dom_tokens_dom_api(b, renderer), state)
    }
}
