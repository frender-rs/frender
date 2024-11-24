mod props_builder;

mod ssr {
    use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_ssr::html::tag::AssertTagName;

    use crate::{
        form_control::{InputDataModel, IntoInputDataModel},
        html::components::input,
    };

    impl<DataModel: IntoInputDataModel> SsrComponent<DataModel> for input::Marker {
        type OneElement<Attrs: IntoSpaceAndHtmlAttributesOrEmpty> = frender_ssr::html::element::VoidElement<
            //
            AssertTagName<&'static str>,
            <(
                //
                Attrs,
                InputDataModel<DataModel::Type, DataModel::Value, DataModel::Checked>,
            ) as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
        >;

        fn ssr_component<Attrs: IntoSpaceAndHtmlAttributesOrEmpty>(self, attrs: Attrs, data_model: DataModel) -> Self::OneElement<Attrs> {
            let data_model = data_model.into_input_data_model();
            Self::OneElement::<Attrs>::new(Self::ASSERT_TAG_NAME, (attrs, data_model).into_space_and_html_attributes_or_empty())
        }
    }
}

mod csr {
    use frender_common::{convert::IntoMut, strings::CsrStr};
    use frender_dom::{
        form_control::{InputChecked, InputType},
        render_state::compound::CompoundState,
    };

    use crate::{
        element_types::RenderStateWithPehKind,
        form_control::{value::FormControlValue, InputDataModel, InputValue, InputValueKind, IntoInputDataModel},
        html::components::input,
        CsrComponent, RenderHtml,
    };

    pin_project_lite::pin_project!(
        #[derive(Debug)]
        pub struct StateWithElementIntoMut<S, E: ?Sized> {
            #[pin]
            inner: S,
            _phantom: std::marker::PhantomData<E>,
        }
    );

    impl<S: Default, E: ?Sized> Default for StateWithElementIntoMut<S, E> {
        fn default() -> Self {
            Self {
                inner: Default::default(),
                _phantom: Default::default(),
            }
        }
    }

    impl<PEH: ?Sized + IntoMut<E>, R: ?Sized, S: frender_dom::RenderStateWithParentElementsHandle<E, R>, E: ?Sized> frender_dom::RenderStateWithParentElementsHandle<PEH, R> for StateWithElementIntoMut<S, E> {
        fn unmount_with_peh(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R) {
            self.project().inner.unmount_with_peh(peh.into_mut(), renderer)
        }

        fn state_unmount_with_peh(self: std::pin::Pin<&mut Self>, peh: &mut PEH) {
            self.project().inner.state_unmount_with_peh(peh.into_mut())
        }

        fn poll_render_with_peh(self: std::pin::Pin<&mut Self>, peh: &mut PEH, renderer: &mut R, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
            self.project().inner.poll_render_with_peh(peh.into_mut(), renderer, cx)
        }
    }

    enum Never {}
    pub struct Kind<Value, Checked, TypeCache>(Never, std::marker::PhantomData<(Value, Checked, TypeCache)>);

    type State<R, Value, Checked, TypeCache> = CompoundState<
        //
        (
            StateWithElementIntoMut<
                <Value as FormControlValue<<Value as InputValue>::ValueKind>>::State<
                    <<Value as InputValue>::ValueKind as InputValueKind>::AsMutFormControlElement<<R as RenderHtml>::input, R>,
                    //
                    R,
                >,
                <<Value as InputValue>::ValueKind as InputValueKind>::AsMutFormControlElement<<R as RenderHtml>::input, R>,
            >,
            <Checked as FormControlValue<bool>>::State<<R as RenderHtml>::input, R>,
        ),
        Option<TypeCache>,
    >;

    impl<Value: InputValue, Checked: InputChecked, TypeCache> RenderStateWithPehKind<input::Marker> for Kind<Value, Checked, TypeCache> {
        type RenderStateWithPeh<R: RenderHtml + ?Sized> = Self::RenderStateWithPehUnpinned<R>;
        type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = State<R, Value, Checked, TypeCache>;
    }

    impl<DataModel: IntoInputDataModel> CsrComponent<DataModel> for input::Marker {
        type ChildrenRenderStateKind = Kind<DataModel::Value, DataModel::Checked, <<DataModel::Type as InputType>::InputTypeStr as CsrStr>::StaticStrCache>;

        fn children_render_update<R: RenderHtml + ?Sized>(
            self,
            children: DataModel,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
        ) {
            self.children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
            self,
            children: DataModel,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: &mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPehUnpinned<R>,
        ) {
            let InputDataModel { r#type, value, checked } = children.into_input_data_model();

            let CompoundState {
                reactive: (state_value, state_checked),
                non_reactive: state_type,
            } = children_state;

            // type should be updated before value is updated
            {
                let input_type = <DataModel::Type>::maybe_into_input_type_str(r#type);

                match (state_type, input_type) {
                    (None, None) => {}
                    (state_type, Some(input_type)) => {
                        _ = frender_common::strings::csr::update_with_option_cache(input_type, state_type, |input_type_str| {
                            use crate::html::behaviors::ElementWithTypeAttribute;

                            element.set_type(renderer, input_type_str);
                        })
                    }
                    (state_type, None) => {
                        use frender_dom::behaviors::Element;

                        element.remove_attribute(renderer, "type");

                        *state_type = None;
                    }
                }
            }

            // value should be updated after type is updated
            {
                let state = &mut state_value.inner;
                <DataModel::Value as FormControlValue<<DataModel::Value as InputValue>::ValueKind>>::update_with_state(value, state, element.into_mut(), renderer)
            }

            {
                <DataModel::Checked as FormControlValue<bool>>::update_with_state(checked, state_checked, element, renderer)
            }
        }
    }
}
