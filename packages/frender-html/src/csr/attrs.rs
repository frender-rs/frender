use std::marker::PhantomData;

use frender_attrs::{experimental::csr::CsrAttributes, IntoAttributes};
use frender_common::convert::FromMut as _;

use crate::{
    csr::behavior_type::{BehaviorType, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    html::{behavior_type_traits, RenderHtml},
};

use frender_dom_values::Attrs;

enum Never {}
pub struct StateKind<S>(Never, PhantomData<S>);

impl<S> UnpinnedNonReactiveRenderStateKind for StateKind<S> {
    type UnpinnedNonReactiveState<R: ?Sized + RenderHtml> = S;
}

impl<
        //
        BT: behavior_type_traits::Element,
        T: IntoAttributes,
    > UnpinnedRenderWithBehavior<BT> for Attrs<T>
{
    type UnpinnedRenderStateKind = StateKind<<T::IntoAttributes as CsrAttributes>::State>;

    fn unpinned_render_init_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
    ) -> <Self::UnpinnedRenderStateKind as crate::csr::behavior_type::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R> {
        this.csr_render_init(renderer, BT::Element::from_mut(b))
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as crate::csr::behavior_type::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        this.csr_render_update(renderer, BT::Element::from_mut(b), state)
    }
}
