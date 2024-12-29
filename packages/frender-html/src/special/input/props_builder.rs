use frender_form_control::input::{InputChecked, InputDataModel, InputType, InputValue, IntoInputDataModel};

use crate::html::components::{input, HtmlInputElement};
use crate::intrinsic::Intrinsic;
use crate::Empty;

impl<DataModel: IntoInputDataModel<Type = Empty>, Attrs, EL> Intrinsic<HtmlInputElement::Marker, DataModel, Attrs, EL> {
    pub fn r#type<V: InputType>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        HtmlInputElement::Marker,
        InputDataModel<V, DataModel::Value, DataModel::Checked>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_type(|Empty| value),
        )
    }

    /// Alias for `r#type`
    pub fn type_<V: InputType>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        HtmlInputElement::Marker,
        InputDataModel<V, DataModel::Value, DataModel::Checked>,
        Attrs,
        EL,
    > {
        self.r#type(value)
    }
}

impl<DataModel: IntoInputDataModel<Type = Empty>, Attrs, EL> Intrinsic<input::Marker, DataModel, Attrs, EL> {
    pub fn r#type<V: InputType>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        input::Marker,
        InputDataModel<V, DataModel::Value, DataModel::Checked>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_type(|Empty| value),
        )
    }

    /// Alias for `r#type`
    pub fn type_<V: InputType>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        input::Marker,
        InputDataModel<V, DataModel::Value, DataModel::Checked>,
        Attrs,
        EL,
    > {
        self.r#type(value)
    }
}

impl<DataModel: IntoInputDataModel<Checked = Empty>, Attrs, EL> Intrinsic<HtmlInputElement::Marker, DataModel, Attrs, EL> {
    pub fn checked<V: InputChecked>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        HtmlInputElement::Marker,
        InputDataModel<DataModel::Type, DataModel::Value, V>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_checked(|Empty| value),
        )
    }
}

impl<DataModel: IntoInputDataModel<Checked = Empty>, Attrs, EL> Intrinsic<input::Marker, DataModel, Attrs, EL> {
    pub fn checked<V: InputChecked>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        input::Marker,
        InputDataModel<DataModel::Type, DataModel::Value, V>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_checked(|Empty| value),
        )
    }
}

impl<DataModel: IntoInputDataModel<Value = Empty>, Attrs, EL> Intrinsic<HtmlInputElement::Marker, DataModel, Attrs, EL> {
    pub fn value<V: InputValue>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        HtmlInputElement::Marker,
        InputDataModel<DataModel::Type, V, DataModel::Checked>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_value(|Empty| value),
        )
    }
}

impl<DataModel: IntoInputDataModel<Value = Empty>, Attrs, EL> Intrinsic<input::Marker, DataModel, Attrs, EL> {
    pub fn value<V: InputValue>(
        self,
        value: V,
    ) -> Intrinsic<
        //
        input::Marker,
        InputDataModel<DataModel::Type, V, DataModel::Checked>,
        Attrs,
        EL,
    > {
        Self::with_map_children(
            //
            self,
            |data| data.into_input_data_model().map_value(|Empty| value),
        )
    }
}
