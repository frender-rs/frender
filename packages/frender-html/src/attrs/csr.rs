use std::marker::PhantomData;

use frender_attrs::{
    experimental::csr::{CsrAttributes, RenderAttributes},
    IntoAttributes,
};
use frender_common::convert::FromMut as _;
use frender_dom::csr::behaviors;

use crate::{
    csr::behavior_type::{BehaviorType, UnpinnedNonReactiveRenderStateKind, UnpinnedRenderWithBehavior},
    html::{behavior_type_traits, RenderHtml},
};

use super::Attrs;

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
        T::into_attributes(this.0).render_init(&mut Render {
            renderer,
            element: BT::Element::from_mut(b),
        })
    }

    fn unpinned_render_update_with_behavior<R: ?Sized + RenderHtml>(
        //
        this: Self,
        renderer: &mut R,
        b: &mut <BT as BehaviorType>::OfBehaviorType<R>,
        state: &mut <Self::UnpinnedRenderStateKind as crate::csr::behavior_type::UnpinnedNonReactiveRenderStateKind>::UnpinnedNonReactiveState<R>,
    ) {
        T::into_attributes(this.0).render_update(
            &mut Render {
                renderer,
                element: BT::Element::from_mut(b),
            },
            state,
        )
    }
}

struct Render<'a, R: ?Sized, E: ?Sized> {
    renderer: &'a mut R,
    element: &'a mut E,
}

impl<R: ?Sized, E: ?Sized + behaviors::Element<R>> RenderAttributes for Render<'_, R, E> {
    fn set_attribute(&mut self, name: &str, value: &str) {
        self.element.set_attribute(self.renderer, name, value)
    }

    fn remove_attribute(&mut self, name: &str) {
        self.element.remove_attribute(self.renderer, name)
    }
}
