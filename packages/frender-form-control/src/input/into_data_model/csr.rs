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

#[cfg(test)]
mod tests {
    use frender_dom::Empty;

    use super::IntoCsrInputDataModel;

    struct Test
    where
        Empty: IntoCsrInputDataModel;
    const _: Test = Test;

    #[test]
    fn compile_only() {
        self::Test = Test;
    }
}
