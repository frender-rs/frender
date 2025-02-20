use frender_dom::Empty;

use super::{InputChecked, InputDataModel, InputType, InputValue};

#[cfg(feature = "csr")]
pub(super) mod csr;
#[cfg(feature = "ssr")]
pub(super) mod ssr;

pub trait IntoInputDataModel {
    type Type: InputType;
    type Value: InputValue;
    type Checked: InputChecked;

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked>;
}

impl IntoInputDataModel for Empty {
    type Type = Empty;
    type Value = Empty;
    type Checked = Empty;

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked> {
        InputDataModel {
            r#type: Empty,
            value: Empty,
            checked: Empty,
        }
    }
}

impl<
        //
        Type: InputType,
        Value: InputValue,
        Checked: InputChecked,
    > IntoInputDataModel for InputDataModel<Type, Value, Checked>
{
    type Type = Type;
    type Value = Value;
    type Checked = Checked;

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked> {
        self
    }
}
