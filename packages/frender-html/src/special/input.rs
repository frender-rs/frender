mod props_builder {
    use frender_html_common::MaybeStringValue;

    use crate::form_control::InputChecked;
    use crate::html::props::HtmlInputElement;
    use crate::Empty;
    use crate::{
        form_control::{InputDataModel, InputValue, IntoInputDataModel},
        props_builder::{PropsBuilderWithChecked, PropsBuilderWithType, PropsBuilderWithValue},
    };

    impl<DataModel: IntoInputDataModel<Type = Empty>, Attrs, EL, V: MaybeStringValue> PropsBuilderWithType<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithType = HtmlInputElement<InputDataModel<V, DataModel::Value, DataModel::Checked>, Attrs, EL>;

        fn r#type(self, value: V) -> Self::WithType {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_type(|Empty| value)),
            }
        }
    }

    impl<DataModel: IntoInputDataModel<Checked = Empty>, Attrs, EL, V: InputChecked> PropsBuilderWithChecked<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithChecked = HtmlInputElement<InputDataModel<DataModel::Type, DataModel::Value, V>, Attrs, EL>;

        fn checked(self, value: V) -> Self::WithChecked {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_checked(|Empty| value)),
            }
        }
    }

    impl<DataModel: IntoInputDataModel<Value = Empty>, V: InputValue, Attrs, EL> PropsBuilderWithValue<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithValue = HtmlInputElement<InputDataModel<DataModel::Type, V, DataModel::Checked>, Attrs, EL>;

        fn value(self, value: V) -> Self::WithValue {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_value(|Empty| value)),
            }
        }
    }
}

mod ssr {
    use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
    use frender_ssr::html::tag::AssertTagName;

    use crate::{
        form_control::{InputDataModel, IntoInputDataModel},
        html::tags,
    };

    impl<
            //
            Attrs: IntoSpaceAndHtmlAttributesOrEmpty,
            DataModel: IntoInputDataModel,
        > SsrComponent<Attrs, DataModel> for tags::input
    {
        type OneElement = frender_ssr::html::element::VoidElement<
            //
            AssertTagName<&'static str>,
            <(
                //
                Attrs,
                InputDataModel<DataModel::Type, DataModel::Value, DataModel::Checked>,
            ) as IntoSpaceAndHtmlAttributesOrEmpty>::SpaceAndHtmlAttributesOrEmpty,
        >;

        fn ssr_component(attrs: Attrs, data_model: DataModel) -> Self::OneElement {
            let data_model = data_model.into_input_data_model();
            Self::OneElement::new(Self::ASSERT_TAG_NAME, (attrs, data_model).into_space_and_html_attributes_or_empty())
        }
    }
}

mod csr {
    use frender_common::convert::IntoMut;
    use frender_dom::{form_control::InputChecked, render_state::compound::CompoundState};
    use frender_html_common::MaybeStringValue;

    use crate::{
        element_types::RenderStateWithPehKind,
        form_control::{value::FormControlValue, InputDataModel, InputValue, InputValueKind, IntoInputDataModel},
        html::tags,
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

    impl<Value: InputValue, Checked: InputChecked, TypeCache> RenderStateWithPehKind<tags::input> for Kind<Value, Checked, TypeCache> {
        type RenderStateWithPeh<R: RenderHtml + ?Sized> = Self::RenderStateWithPehUnpinned<R>;
        type RenderStateWithPehUnpinned<R: RenderHtml + ?Sized> = State<R, Value, Checked, TypeCache>;
    }

    impl<DataModel: IntoInputDataModel> CsrComponent<DataModel> for tags::input {
        type ChildrenRenderStateKind = Kind<DataModel::Value, DataModel::Checked, <DataModel::Type as MaybeStringValue>::StringValue>;

        fn children_render_update<R: RenderHtml + ?Sized>(
            children: DataModel,
            element: &mut Self::Element<R>,
            renderer: &mut R,
            children_state: std::pin::Pin<&mut <Self::ChildrenRenderStateKind as RenderStateWithPehKind<Self>>::RenderStateWithPeh<R>>,
        ) {
            Self::children_unpinned_render_update(children, element, renderer, children_state.get_mut())
        }

        fn children_unpinned_render_update<R: RenderHtml + ?Sized>(
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
                let input_type = <DataModel::Type as MaybeStringValue>::maybe_string_value(r#type);

                let input_type_str = input_type.as_ref().map(AsRef::as_ref);
                if state_type.as_ref().map(AsRef::as_ref) != input_type_str {
                    use frender_dom::behaviors::Element;

                    use crate::html::behaviors::ElementWithTypeAttribute;

                    if let Some(input_type_str) = input_type_str {
                        element.set_type(renderer, input_type_str);
                    } else {
                        element.remove_attribute(renderer, "type");
                    }

                    *state_type = input_type;
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
