pub mod inner_html;
pub mod input;
pub mod script;
pub mod style;
mod textarea;
mod void_elements;

mod utils {
    use frender_attr_value::csr::UpdateAttrValue;

    pub(super) struct UpdateInnerText<'a, E: ?Sized, R: ?Sized>(pub(super) &'a mut E, pub(super) &'a mut R);

    impl<'a, E: frender_dom::behaviors::HtmlElement<R> + ?Sized, R: ?Sized> UpdateAttrValue for UpdateInnerText<'a, E, R> {
        type Kind = str;
        fn set(self, value: &str) {
            self.0.set_inner_text(self.1, value)
        }

        fn remove(self) {
            self.0.set_inner_text(self.1, "")
        }
    }
}

macro_rules! define_Kind_non_reactive {
    (
        pub struct $Kind:ident;
        trait BehaviorTypeTrait = $BehaviorTypeTrait:path;
    ) => {
        enum Never {}
        pub struct $Kind<T>(Never, ::core::marker::PhantomData<T>);

        impl<T> crate::element::UnpinnedRenderStateKind for $Kind<T> {
            type UnpinnedUiHandle<R: crate::RenderHtml + ?Sized> = ();
            type UnpinnedNonReactiveState<R: crate::RenderHtml + ?Sized> = T;
            type UnpinnedReactiveState = ();
        }

        impl<T> crate::element::PinnedRenderStateKind for $Kind<T> {
            type PinnedUiHandle<R: crate::RenderHtml + ?Sized> = crate::kinds::UiHandleWithNonReactiveState<(), T>;
            type PinnedNonReactiveState<R: crate::RenderHtml + ?Sized> = ();
            type PinnedReactiveState = ();
        }

        impl<T, ET: ?Sized + $BehaviorTypeTrait> crate::element_types::RenderStateKindPollRenderWithParent<ET> for $Kind<T> {
            fn pinned_poll_render_with_parent<R: crate::RenderHtml + ?Sized>(
                //
                _: &mut R,
                _: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
                _: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
                _: &mut std::task::Context<'_>,
            ) -> ::core::task::Poll<()> {
                ::core::task::Poll::Ready(())
            }

            fn unpinned_poll_render_with_parent<R: crate::RenderHtml + ?Sized>(
                //
                _: &mut R,
                _: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
                _: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
                _: &mut std::task::Context<'_>,
            ) -> ::core::task::Poll<()> {
                ::core::task::Poll::Ready(())
            }
        }
    };
}

use define_Kind_non_reactive;

macro_rules! define_Kind_with_ReactiveValue {
    (
        pub struct $Kind:ident;
        type ReactiveValueKind = $ReactiveValueKind:ty;
        type $BehaviorType:ident : $BehaviorTypeTrait:path;

        const into_mut_renderer: _ = for<$Renderer:ident> |
            $renderer:pat_param,
            $parent:pat_param $(,)?
        | $into_mut_renderer:expr;
    ) => {
        enum Never {}

        pub struct $Kind<P, U>(Never, ::core::marker::PhantomData<(P, U)>);

        const _: () = {
            use frender_common::{csr::StateUnmount, reactive_value::ReactiveValueState};

            impl<P, U: StateUnmount + Default + Unpin> crate::element::UnpinnedRenderStateKind for $Kind<P, U> {
                type UnpinnedUiHandle<R: crate::RenderHtml + ?Sized> = ();
                type UnpinnedNonReactiveState<R: crate::RenderHtml + ?Sized> = ();
                type UnpinnedReactiveState = U;
            }
            impl<P: StateUnmount + Default, U> crate::element::PinnedRenderStateKind for $Kind<P, U> {
                type PinnedUiHandle<R: crate::RenderHtml + ?Sized> = ();
                type PinnedNonReactiveState<R: crate::RenderHtml + ?Sized> = ();
                type PinnedReactiveState = P;
            }

            impl<
                    //
                    P: Default + ReactiveValueState<ReactiveValueKind = $ReactiveValueKind>,
                    U: Default + Unpin + ReactiveValueState<ReactiveValueKind = $ReactiveValueKind>,
                    $BehaviorType: ?Sized + $BehaviorTypeTrait,
                > crate::element_types::RenderStateKindPollRenderWithParent<$BehaviorType> for Kind<P, U>
            {
                fn pinned_poll_render_with_parent<$Renderer: crate::RenderHtml + ?Sized>(
                    $renderer: &mut $Renderer,
                    $parent: &mut <$BehaviorType as crate::BehaviorType>::OfBehaviorType<R>,
                    crate::element::RenderStates {
                        ui_handle: (),
                        non_reactive_state: _,
                        reactive_state,
                    }: crate::element::PinnedMutRenderStates<(), (), P>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<()> {
                    <P as ReactiveValueState>::reactive_value_state_poll_render(reactive_state, $into_mut_renderer, cx)
                }

                fn unpinned_poll_render_with_parent<$Renderer: crate::RenderHtml + ?Sized>(
                    $renderer: &mut $Renderer,
                    $parent: &mut <$BehaviorType as crate::BehaviorType>::OfBehaviorType<R>,
                    crate::element::RenderStates {
                        ui_handle: (),
                        non_reactive_state: (),
                        reactive_state,
                    }: crate::element::UnpinnedMutRenderStates<(), (), U>,
                    cx: &mut std::task::Context<'_>,
                ) -> std::task::Poll<()> {
                    <U as ReactiveValueState>::reactive_value_state_poll_render(std::pin::Pin::new(reactive_state), $into_mut_renderer, cx)
                }
            }
        };
    };
}

macro_rules! impl_CsrComponent_with_ReactiveValue {
    (
        type Kind = $Kind:ident;

        type This = $This:ty;

        const into_reactive_value: $ReactiveValue:ident =
            |$this_pat:pat_param| $into_reactive_value:expr;

        const into_renderer: _ = for<$Renderer:ident> |
            $renderer:pat_param,
            $parent:pat_param $(,)?
        | $into_renderer:expr;
    ) => {
        type ChildrenRenderStateKind = $Kind<$ReactiveValue::PinnedStateDefault, $ReactiveValue::UnpinnedStateDefault>;

        fn children_pinned_render_init<$Renderer: crate::RenderHtml + ?Sized>(
            self,
            $this_pat: $This,
            $renderer: &mut $Renderer,
            $parent: &mut Self::OfBehaviorType<$Renderer>,
            crate::element::PinMutRenderInitStates {
                //
                non_reactive_state: _,
                reactive_state,
            }: crate::element::PinMutRenderInitStates<'_, (), $ReactiveValue::PinnedStateDefault>,
        ) -> crate::element::PinnedUiHandleOfKind<$Renderer, Self::ChildrenRenderStateKind> {
            frender_common::reactive_value::ReactiveValue::reactive_value_render_init_pinned($into_reactive_value, $into_renderer, reactive_state)
        }

        fn children_pinned_render_update<$Renderer: crate::RenderHtml + ?Sized>(
            //
            self,
            $this_pat: $This,
            $renderer: &mut $Renderer,
            $parent: &mut Self::OfBehaviorType<$Renderer>,
            crate::element::RenderStates {
                ui_handle: (),
                non_reactive_state: _,
                reactive_state,
            }: crate::element::PinnedMutRenderStates<(), (), $ReactiveValue::PinnedStateDefault>,
        ) {
            let _: Option<()> = frender_common::reactive_value::ReactiveValue::reactive_value_render_update_pinned($into_reactive_value, $into_renderer, reactive_state);
        }

        fn children_unpinned_render_init<$Renderer: crate::RenderHtml + ?Sized>(
            //
            self,
            $this_pat: $This,
            $renderer: &mut $Renderer,
            $parent: &mut Self::OfBehaviorType<$Renderer>,
        ) -> crate::element::RenderStates<(), (), $ReactiveValue::UnpinnedStateDefault> {
            let (reactive_state, ()) = frender_common::reactive_value::ReactiveValue::reactive_value_render_init_unpinned($into_reactive_value, $into_renderer);
            crate::element::RenderStates {
                ui_handle: (),
                non_reactive_state: (),
                reactive_state: <$ReactiveValue::UnpinnedStateDefault as From<$ReactiveValue::UnpinnedState>>::from(reactive_state),
            }
        }

        fn children_unpinned_render_update<$Renderer: crate::RenderHtml + ?Sized>(
            //
            self,
            $this_pat: $This,
            $renderer: &mut $Renderer,
            $parent: &mut Self::OfBehaviorType<$Renderer>,
            crate::element::RenderStates {
                ui_handle: (),
                non_reactive_state: (),
                reactive_state,
            }: crate::element::UnpinnedMutRenderStates<(), (), $ReactiveValue::UnpinnedStateDefault>,
        ) {
            use frender_common::reactive_value::AsOptionMut;
            let Some(reactive_state) = AsOptionMut::<$ReactiveValue::UnpinnedState>::as_option_mut(reactive_state) else {
                panic!("children_unpinned_render_update must be called with initialized render states")
            };
            let _: Option<()> = frender_common::reactive_value::ReactiveValue::reactive_value_render_update_unpinned($into_reactive_value, $into_renderer, reactive_state);
        }
    };
}

use {define_Kind_with_ReactiveValue, impl_CsrComponent_with_ReactiveValue};
