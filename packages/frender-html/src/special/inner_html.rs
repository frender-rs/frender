pub mod csr {
    use std::{marker::PhantomData, pin::Pin, task::Poll};

    use frender_common::reactive_value::{ReactiveValue, ReactiveValueState, RenderValueWithFnMutAndData};
    use frender_dom::{special::DangerousInnerHtml, StateUnmount};

    use crate::{
        element::{PinMutRenderInitStates, PinnedRenderStateKind, RenderStates, UnpinnedRenderStateKind},
        element_types::RenderStateKindPollRenderWithParent,
        html::behavior_type_traits,
        CsrComponent, CsrComponentNormalElement, RenderHtml,
    };

    enum Never {}

    pub struct Kind<P, U>(Never, PhantomData<(P, U)>);

    impl<P, U: StateUnmount + Default + Unpin> UnpinnedRenderStateKind for Kind<P, U> {
        type UnpinnedUiHandle<R: RenderHtml + ?Sized> = ();
        type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
        type UnpinnedReactiveState = U;
    }
    impl<P: StateUnmount + Default, U> PinnedRenderStateKind for Kind<P, U> {
        type PinnedUiHandle<R: RenderHtml + ?Sized> = ();
        type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
        type PinnedReactiveState = P;
    }

    fn render_value<'a, ET: ?Sized + behavior_type_traits::Element, R: RenderHtml + ?Sized>(
        //
        renderer: &'a mut R,
        parent: &'a mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
    ) -> RenderValueWithFnMutAndData<
        //
        impl FnMut(&mut (&'a mut R, &'a mut <ET as crate::BehaviorType>::OfBehaviorType<R>), &str),
        impl FnMut(&mut (&'a mut R, &'a mut <ET as crate::BehaviorType>::OfBehaviorType<R>)),
        (&'a mut R, &'a mut <ET as crate::BehaviorType>::OfBehaviorType<R>),
    > {
        RenderValueWithFnMutAndData {
            update: |(renderer, parent): &mut (&'a mut _, &'a mut _), value: &str| update_inner_html::<ET, R>(renderer, parent, value),
            remove: |(renderer, parent): &mut (&'a mut _, &'a mut _)| update_inner_html::<ET, R>(renderer, parent, ""),
            data: (renderer, parent),
        }
    }

    impl<P: Default + ReactiveValueState<ReactiveValueKind = str>, U: Default + Unpin + ReactiveValueState<ReactiveValueKind = str>, ET: ?Sized + behavior_type_traits::Element> RenderStateKindPollRenderWithParent<ET>
        for Kind<P, U>
    {
        fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
            //
            renderer: &mut R,
            parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
            RenderStates {
                ui_handle: (),
                non_reactive_state: _,
                reactive_state,
            }: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            reactive_state.reactive_value_state_poll_render(&mut render_value::<ET, R>(renderer, parent), cx)
        }

        fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
            //
            renderer: &mut R,
            parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
            RenderStates {
                ui_handle: (),
                non_reactive_state: (),
                reactive_state,
            }: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<()> {
            Pin::new(reactive_state).reactive_value_state_poll_render(&mut render_value::<ET, R>(renderer, parent), cx)
        }
    }

    fn update_inner_html<ET: ?Sized + behavior_type_traits::Element, R: RenderHtml + ?Sized>(
        //
        renderer: &mut R,
        parent: &mut ET::OfBehaviorType<R>,
        value: &str,
    ) {
        use frender_common::convert::FromMut as _;
        use frender_dom::behaviors::Element as _;

        <<ET as behavior_type_traits::Element>::Element<R>>::from_mut(parent).set_inner_html(renderer, value)
    }

    impl<ET: CsrComponentNormalElement, S: ReactiveValue<str>> CsrComponent<DangerousInnerHtml<S>> for ET {
        super::super::impl_CsrComponent_with_ReactiveValue!(
            type Kind = Kind;
            type This = DangerousInnerHtml<S>;
            const into_reactive_value: S = |DangerousInnerHtml(inner_html)| inner_html;
            const into_renderer: _ = for<R> |renderer, parent| render_value::<ET, R>(renderer, parent);
        );
    }
}
