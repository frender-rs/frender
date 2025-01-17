mod props_builder {
    use frender_form_control::textarea::TextAreaValue;

    use crate::html::components::{textarea, HtmlTextAreaElement};

    impl<Attrs, EL> HtmlTextAreaElement::Props<crate::Empty, Attrs, EL> {
        /// Alias for [`Self::children`]
        pub fn value<V: TextAreaValue>(self, value: V) -> HtmlTextAreaElement::Props<V, Attrs, EL> {
            self.children(value)
        }
    }

    impl<Attrs, EL> textarea::Element<crate::Empty, Attrs, EL> {
        /// Alias for [`Self::children`]
        pub fn value<V: TextAreaValue>(self, value: V) -> textarea::Element<V, Attrs, EL> {
            self.children(value)
        }
    }
}

pub mod ssr {
    use frender_dom::component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_form_control::textarea::SsrTextAreaValue;
    use frender_ssr::html::tag::AssertTagName;

    use crate::cs::textarea;

    type Element<Attrs, Children> = frender_ssr::html::element::NormalElement<
        //
        AssertTagName<&'static str>,
        Attrs,
        Children,
    >;

    impl<Children> SsrComponent<Children> for textarea::Marker
    where
        Children: SsrTextAreaValue,
    {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = Element<
            //
            Attrs::SpaceAndHtmlAttributesOrEmpty,
            Children::IntoSsrTextAreaValue,
        >;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
            use frender_dom::component::HasIntrinsicComponentTag;
            frender_ssr::html::element::NormalElement::new(Self::ASSERT_TAG_NAME, attrs.into_space_and_html_attributes_or_empty(), Children::into_ssr_text_area_value(children))
        }
    }
}

pub mod csr {
    use std::marker::PhantomData;

    use frender_common::convert::FromMut as _;
    use frender_form_control::value::FormControlValueStateKind;

    use crate::{
        cs::textarea,
        element::{PinMutRenderInitStates, PinnedRenderStateKind, RenderStates, UnpinnedRenderStateKind},
        element_types::RenderStateKindPollRenderWithParent,
        form_control::value::FormControlValue,
        html::behavior_type_traits,
        kinds::UiHandleWithNonReactiveState,
        CsrComponent, RenderHtml,
    };

    enum Never {}
    pub struct Kind<K, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement>(Never, PhantomData<K>, PhantomData<ET>);

    impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> UnpinnedRenderStateKind for Kind<K, ET> {
        type UnpinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<(), K::UnpinnedNonReactiveState<ET::HtmlTextAreaElement<R>, R>>;
        type UnpinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
        type UnpinnedReactiveState = K::UnpinnedReactiveState;
    }

    impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> PinnedRenderStateKind for Kind<K, ET> {
        type PinnedUiHandle<R: RenderHtml + ?Sized> = UiHandleWithNonReactiveState<(), K::UnpinnedNonReactiveState<ET::HtmlTextAreaElement<R>, R>>;
        type PinnedNonReactiveState<R: RenderHtml + ?Sized> = ();
        type PinnedReactiveState = K::UnpinnedReactiveState;
    }

    impl<K: FormControlValueStateKind<str>, ET: ?Sized + behavior_type_traits::HtmlTextAreaElement> RenderStateKindPollRenderWithParent<ET> for Kind<K, ET> {
        fn pinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
            renderer: &mut R,
            parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            }: crate::element::PinnedMutRenderStatesOfKind<Self, R>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            <Self as RenderStateKindPollRenderWithParent<ET>>::unpinned_poll_render_with_parent(
                renderer,
                parent,
                RenderStates {
                    ui_handle,
                    non_reactive_state: non_reactive_state.get_mut(),
                    reactive_state: reactive_state.get_mut(),
                },
                cx,
            )
        }

        fn unpinned_poll_render_with_parent<R: RenderHtml + ?Sized>(
            //
            renderer: &mut R,
            parent: &mut <ET as crate::BehaviorType>::OfBehaviorType<R>,
            RenderStates {
                ui_handle: UiHandleWithNonReactiveState { ui_handle: (), non_reactive_state },
                non_reactive_state: (),
                reactive_state,
            }: crate::element::UnpinnedMutRenderStatesOfKind<Self, R>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<()> {
            K::unpinned_poll_render_form_control_value_state(
                //
                renderer,
                ET::HtmlTextAreaElement::from_mut(parent),
                non_reactive_state,
                reactive_state,
                cx,
            )
        }
    }

    impl<Children> CsrComponent<Children> for textarea::Marker
    where
        Children: FormControlValue<str>,
    {
        type ChildrenRenderStateKind = Kind<Children::StateKind, Self>;

        fn children_pinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            PinMutRenderInitStates { non_reactive_state, reactive_state }: crate::element::PinMutRenderInitStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
            let ui_handle;
            RenderStates {
                ui_handle,
                non_reactive_state: *non_reactive_state.get_mut(),
                reactive_state: *reactive_state.get_mut(),
            } = self.children_unpinned_render_init(children, renderer, parent);
            ui_handle
        }

        fn children_pinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            RenderStates {
                ui_handle,
                non_reactive_state,
                reactive_state,
            }: crate::element::PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
            self.children_unpinned_render_update(
                children,
                renderer,
                parent,
                RenderStates {
                    ui_handle,
                    non_reactive_state: non_reactive_state.get_mut(),
                    reactive_state: reactive_state.get_mut(),
                },
            );
        }

        fn children_unpinned_render_init<R: RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
        ) -> crate::element::UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R> {
            let (non_reactive_state, reactive_state) = Children::render_init(children, renderer, R::textarea::from_mut(parent));
            RenderStates {
                ui_handle: UiHandleWithNonReactiveState { ui_handle: (), non_reactive_state },
                non_reactive_state: (),
                reactive_state,
            }
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            RenderStates {
                ui_handle: UiHandleWithNonReactiveState { ui_handle: (), non_reactive_state },
                non_reactive_state: (),
                reactive_state,
            }: crate::element::UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
            Children::render_update(children, renderer, R::textarea::from_mut(parent), non_reactive_state, reactive_state)
        }
    }
}
