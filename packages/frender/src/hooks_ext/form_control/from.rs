use std::borrow::Cow;

use frender_html::form_control::{
    FormControlValueKind, KindOfChecked, KindOfValue, KindOfValueAsNumber,
};

pub trait FromFormControlValue<VK: ?Sized + FormControlValueKind> {
    fn from_form_control_value(v: VK::FormControlValue<'_>) -> Self;
}

impl FromFormControlValue<KindOfValue> for String {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned()
    }
}

impl FromFormControlValue<KindOfValue> for Cow<'_, str> {
    fn from_form_control_value(v: Cow<'_, str>) -> Self {
        v.into_owned().into()
    }
}

frender_common::impl_many!(
    impl<__> FromFormControlValue<KindOfValue>
        for each_of![
            //
            std::rc::Rc<str>,
            std::sync::Arc<str>
        ]
    {
        fn from_form_control_value(v: Cow<'_, str>) -> Self {
            v.into()
        }
    }
);

impl FromFormControlValue<KindOfChecked> for bool {
    fn from_form_control_value(v: bool) -> Self {
        v
    }
}

impl FromFormControlValue<KindOfValueAsNumber> for f64 {
    fn from_form_control_value(v: f64) -> Self {
        v
    }
}
