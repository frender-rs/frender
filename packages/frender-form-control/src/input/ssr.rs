use async_str_iter::{
    any_str::IterAnyStr, chain::Chain, option::IterOption, IntoAsyncStrIterator as _,
};
use frender_common::{strings::SsrStr, IntoStaticStr};
use frender_dom::component::IntoSpaceAndHtmlAttributesOrEmpty;
use frender_ssr::html::{
    attr::{AssertSpaceAndHtmlAttributeName, SpaceAndHtmlAttribute},
    attr_value::AttrEqValue,
};

use crate::value::{MaybeProvideFormControlValue, ProvideFormControlValue as _};

use super::{InputChecked, InputDataModel, InputType, InputValue, InputValueKind};

impl<
        //
        Type: InputType,
        Value: InputValue,
        Checked: InputChecked,
    > IntoSpaceAndHtmlAttributesOrEmpty for InputDataModel<Type, Value, Checked>
{
    type SpaceAndHtmlAttributesOrEmpty = Chain<
        // type
        IterOption<
            SpaceAndHtmlAttribute<
                //
                AssertSpaceAndHtmlAttributeName<&'static str>,
                AttrEqValue<IterAnyStr<
                    <<Type as InputType>::InputTypeStr as SsrStr>::StaticStr
                >>,
            >,
        >,
        Chain<
            // value
            IterOption<
                SpaceAndHtmlAttribute<
                    //
                    AssertSpaceAndHtmlAttributeName<&'static str>,
                    <Value::ValueKind as InputValueKind>::IntoInputValueAttrValue<
                        //
                        <Value as MaybeProvideFormControlValue<Value::ValueKind>>::ProvideFormControlValue,
                    >,
                >,
            >,
            // checked
            IterOption<AssertSpaceAndHtmlAttributeName<&'static str>>,
        >,
    >;

    fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
        const TYPE: AssertSpaceAndHtmlAttributeName<&'static str> =
            AssertSpaceAndHtmlAttributeName::new_from_str(" type");
        const VALUE: AssertSpaceAndHtmlAttributeName<&'static str> =
            AssertSpaceAndHtmlAttributeName::new_from_str(" value");
        const CHECKED: AssertSpaceAndHtmlAttributeName<&'static str> =
            AssertSpaceAndHtmlAttributeName::new_from_str(" checked");

        let Self {
            r#type: input_type,
            value,
            checked,
        } = self;

        let input_type = Type::maybe_into_input_type_str(input_type)
            .map(|v| v.into_into_static_str().into_static_str());

        let value_attr = Value::maybe_into_provide_form_control_value(value)
            .map(|value| {
                let input_type = input_type.as_ref().map_or("", |v| v.as_ref());
                Value::ValueKind::into_input_value_attr_value(value, input_type)
            })
            .map(|eq_value| SpaceAndHtmlAttribute(VALUE, eq_value));

        let checked_attr = {
            let checked = Checked::maybe_into_provide_form_control_value(checked)
                .map_or(false, |checked| checked.provide_form_control_value(|v| *v));
            checked.then_some(CHECKED)
        };

        Chain::new(
            input_type
                .map(|input_type| {
                    let value = IterAnyStr::new(input_type);
                    let value = AttrEqValue(value);
                    SpaceAndHtmlAttribute(TYPE, value)
                })
                .into_async_str_iterator(),
            Chain::new(
                value_attr.into_async_str_iterator(),
                checked_attr.into_async_str_iterator(),
            ),
        )
    }
}
