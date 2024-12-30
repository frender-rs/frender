use frender_dom::component::{HasIntrinsicComponentTag, IntoSpaceAndHtmlAttributesOrEmpty, SsrComponent};
use frender_ssr::html::tag::AssertTagName;

use frender_form_control::input::{InputDataModel, IntoInputDataModel};

use crate::html::components::input;

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
