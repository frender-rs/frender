use super::{InputChecked, InputType, InputValue};

pub struct InputDataModel<Type: InputType, Value: InputValue, Checked: InputChecked> {
    pub r#type: Type,
    pub value: Value,
    pub checked: Checked,
}

impl<Type: InputType, Value: InputValue, Checked: InputChecked>
    InputDataModel<Type, Value, Checked>
{
    pub fn map_type<V: InputType>(
        self,
        f: impl FnOnce(Type) -> V,
    ) -> InputDataModel<V, Value, Checked> {
        let Self {
            r#type,
            value,
            checked,
        } = self;
        InputDataModel {
            r#type: f(r#type),
            value,
            checked,
        }
    }

    pub fn map_value<V: InputValue>(
        self,
        f: impl FnOnce(Value) -> V,
    ) -> InputDataModel<Type, V, Checked> {
        let Self {
            r#type,
            value,
            checked,
        } = self;
        InputDataModel {
            r#type,
            value: f(value),
            checked,
        }
    }

    pub fn map_checked<V: InputChecked>(
        self,
        f: impl FnOnce(Checked) -> V,
    ) -> InputDataModel<Type, Value, V> {
        let Self {
            r#type,
            value,
            checked,
        } = self;
        InputDataModel {
            r#type,
            value,
            checked: f(checked),
        }
    }
}
