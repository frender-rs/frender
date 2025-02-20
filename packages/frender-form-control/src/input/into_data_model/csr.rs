use crate::input::{CsrInputChecked, CsrInputType, CsrInputValue};

use super::IntoInputDataModel;

pub trait IntoCsrInputDataModel:
    IntoInputDataModel<Type: CsrInputType, Value: CsrInputValue, Checked: CsrInputChecked>
{
}

impl<T: ?Sized> IntoCsrInputDataModel for T where
    T: IntoInputDataModel<Type: CsrInputType, Value: CsrInputValue, Checked: CsrInputChecked>
{
}
