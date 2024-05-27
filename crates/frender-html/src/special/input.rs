mod props_builder {
    use frender_html_common::MaybeStringValue;

    use crate::form_control::InputChecked;
    use crate::html::props::HtmlInputElement;
    use crate::{
        form_control::{value::FormControlValue, InputDataModel, InputValue, IntoInputDataModel},
        props_builder::{PropsBuilderWithChecked, PropsBuilderWithType, PropsBuilderWithValue},
    };

    impl<DataModel: IntoInputDataModel<Type = ()>, Attrs, EL, V: MaybeStringValue> PropsBuilderWithType<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithType = HtmlInputElement<InputDataModel<V, DataModel::Value, DataModel::Checked>, Attrs, EL>;

        fn r#type(self, value: V) -> Self::WithType {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_type(|()| value)),
            }
        }
    }

    impl<DataModel: IntoInputDataModel<Checked = ()>, Attrs, EL, V: InputChecked> PropsBuilderWithChecked<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithChecked = HtmlInputElement<InputDataModel<DataModel::Type, DataModel::Value, V>, Attrs, EL>;

        fn checked(self, value: V) -> Self::WithChecked {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_checked(|()| value)),
            }
        }
    }

    impl<DataModel: IntoInputDataModel<Value = ()>, V: InputValue, Attrs, EL> PropsBuilderWithValue<V> for HtmlInputElement<DataModel, Attrs, EL> {
        type WithValue = HtmlInputElement<InputDataModel<DataModel::Type, V, DataModel::Checked>, Attrs, EL>;

        fn value(self, value: V) -> Self::WithValue {
            HtmlInputElement {
                props: self.props.map_children(|data| data.into_input_data_model().map_value(|()| value)),
            }
        }
    }
}

mod ssr {
    use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent, SsrComponentNormalElement};
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
