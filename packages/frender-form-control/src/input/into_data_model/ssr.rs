use crate::input::{SsrInputChecked, SsrInputType, SsrInputValue};

use super::IntoInputDataModel;

pub trait IntoSsrInputDataModel:
    IntoInputDataModel<Type: SsrInputType, Value: SsrInputValue, Checked: SsrInputChecked>
{
}

impl<T: ?Sized> IntoSsrInputDataModel for T where
    T: IntoInputDataModel<Type: SsrInputType, Value: SsrInputValue, Checked: SsrInputChecked>
{
}
