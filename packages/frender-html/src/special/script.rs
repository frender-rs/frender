mod ssr {
    use frender_dom::{
        component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
        script::IntoScriptContent,
    };

    use crate::html::components::script;

    impl<Children: IntoScriptContent> SsrComponent<Children> for script::Marker {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::ScriptElement<<Attrs as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty, Children::IntoScriptContent>;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, children: Children) -> Self::OneElement<Attrs> {
            Self::OneElement::<Attrs>::new(attrs.into_space_and_html_attributes_or_empty(), IntoScriptContent::into_script_content(children))
        }
    }
}

pub mod csr {
    use frender_attr_value::csr::CsrAttrValue;
    use frender_dom::script::IntoScriptContent;

    use crate::{
        element::{PinMutRenderInitStates, RenderStates},
        html::{behavior_type_traits, components::script},
        kinds::UiHandleWithNonReactiveState,
        CsrComponent,
    };

    super::super::define_Kind_non_reactive!(
        pub struct Kind;
        trait BehaviorTypeTrait = behavior_type_traits::HtmlScriptElement;
    );

    impl<Children: IntoScriptContent> CsrComponent<Children> for script::Marker {
        type ChildrenRenderStateKind = Kind<<Children::IntoScriptInnerText as CsrAttrValue<str>>::State>;

        fn children_pinned_render_init<R: crate::RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            _: PinMutRenderInitStates<(), ()>,
        ) -> crate::element::PinnedUiHandleOfKind<R, Self::ChildrenRenderStateKind> {
            let RenderStates {
                ui_handle: (),
                non_reactive_state,
                reactive_state: (),
            } = self.children_unpinned_render_init(children, renderer, parent);
            UiHandleWithNonReactiveState { ui_handle: (), non_reactive_state }
        }

        fn children_pinned_render_update<R: crate::RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            RenderStates {
                ui_handle: UiHandleWithNonReactiveState { ui_handle, non_reactive_state },
                non_reactive_state: _,
                reactive_state,
            }: crate::element::PinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
            self.children_unpinned_render_update(
                children,
                renderer,
                parent,
                RenderStates {
                    ui_handle,
                    non_reactive_state,
                    reactive_state: reactive_state.get_mut(),
                },
            )
        }

        fn children_unpinned_render_init<R: crate::RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
        ) -> crate::element::UnpinnedRenderStatesOfKind<Self::ChildrenRenderStateKind, R> {
            let children = Children::into_script_inner_text(children);
            let updater = super::super::utils::UpdateInnerText(parent, renderer);

            RenderStates {
                ui_handle: (),
                non_reactive_state: <_>::update_absent_attribute_value_into_state(children, updater),
                reactive_state: (),
            }
        }

        fn children_unpinned_render_update<R: crate::RenderHtml + ?Sized>(
            //
            self,
            children: Children,
            renderer: &mut R,
            parent: &mut Self::OfBehaviorType<R>,
            RenderStates {
                ui_handle: (),
                non_reactive_state,
                reactive_state: (),
            }: crate::element::UnpinnedMutRenderStatesOfKind<Self::ChildrenRenderStateKind, R>,
        ) {
            let children = Children::into_script_inner_text(children);
            let updater = super::super::utils::UpdateInnerText(parent, renderer);

            <_>::update_attribute_value_with_state(children, updater, non_reactive_state)
        }
    }
}
