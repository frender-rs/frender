mod props_builder {
    use frender_dom::value::FormControlValue;

    use crate::html::props::HtmlTextAreaElement;

    impl<Attrs> HtmlTextAreaElement::Building<(), Attrs> {
        /// Alias for [`Self::children`]
        pub fn value<Children: FormControlValue>(self, value: Children) -> HtmlTextAreaElement::Building<Children, Attrs> {
            self.children(value)
        }
    }
}

pub mod ssr {
    use frender_dom::{
        component::{IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent},
        value::FormControlValue,
    };
    use frender_ssr::html::{encode::Encode, escape_safe, tag::AssertTagName};

    use crate::html::tags;

    type Element<Attrs, Children> = frender_ssr::html::element::NormalElement<
        //
        AssertTagName<&'static str>,
        Attrs,
        Encode<escape_safe::Safe, Children>,
    >;

    impl<Attrs: IntoSpaceAndHtmlAttributesOrEmpty, Children> SsrComponent<Attrs, Children> for tags::textarea
    where
        Children: FormControlValue,
    {
        type OneElement = Element<
            //
            Attrs::SpaceAndHtmlAttributesOrEmpty,
            Children::OneStringOrEmpty,
        >;

        fn ssr_component(attrs: Attrs, children: Children) -> Self::OneElement {
            use frender_dom::component::HasIntrinsicComponentTag;
            frender_ssr::html::element::NormalElement::new(
                Self::ASSERT_TAG_NAME,
                attrs.into_space_and_html_attributes_or_empty(),
                Encode::new(escape_safe::Safe, Children::into_one_string_or_empty(children)),
            )
        }
    }
}

pub mod csr {
    use frender_dom::value::{FormControlController, FormControlValue};

    use crate::{
        elements::non_reactive::NonReactiveRenderState,
        html::{behaviors, tags},
        CsrComponent, RenderHtml,
    };

    impl<Children> CsrComponent<Children> for tags::textarea
    where
        Children: FormControlValue,
    {
        type ChildrenRenderState<R: RenderHtml> = Self::ChildrenUnpinnedRenderState<R>;

        fn children_render_update<R: RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: std::pin::Pin<&mut Self::ChildrenRenderState<R>>) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        type ChildrenUnpinnedRenderState<R: RenderHtml> = NonReactiveRenderState<
            Children::State<
                //
                <R::textarea as frender_dom::behaviors::HtmlElement<R>>::OnBeforeInputPreventDefault,
                <R::textarea as behaviors::HtmlElement<R>>::OnInputEventListener,
            >,
        >;

        fn children_unpinned_render_update<R: RenderHtml>(children: Children, element: &mut Self::Element<R>, renderer: &mut R, children_state: &mut Self::ChildrenUnpinnedRenderState<R>) {
            let element: &mut R::textarea = element;
            Children::update_with_state(children, &mut children_state.0, controller::HtmlTextAreaElementValue { element, renderer })
        }
    }

    pub mod controller {
        use frender_dom::value::FormControlController;
        use frender_events::event::Event;

        use crate::html::behaviors;

        pub struct HtmlTextAreaElementValue<'a, E: behaviors::HtmlTextAreaElement<R>, R> {
            pub(super) element: &'a mut E,
            pub(super) renderer: &'a mut R,
        }

        impl<'a, E: behaviors::HtmlTextAreaElement<R>, R> FormControlController for HtmlTextAreaElementValue<'a, E, R> {
            fn set_default_value(&mut self, value: &str) {
                self.element.set_default_value(self.renderer, value)
            }

            fn set_value(&mut self, value: &str) {
                self.element.set_value(self.renderer, value)
            }

            type ForceValue = E::OnBeforeInputPreventDefault;

            fn force_value<V: frender_dom::value::TempAsRef<str> + 'static>(&mut self, value: V) -> Self::ForceValue {
                self.element.on_before_input_prevent_default(self.renderer)
            }

            type OnValueChangeEventListener = E::OnInputEventListener;

            fn on_value_change(&mut self, mut f: impl FnMut(std::borrow::Cow<'_, str>) + 'static) -> Self::OnValueChangeEventListener {
                self.element.on_input(self.renderer, move |e| {
                    if let Some(value) = e.target_form_control_value() {
                        f(value)
                    } else {
                        // TODO: warning about unexpected event.target
                    }
                })
            }
        }
    }
}
