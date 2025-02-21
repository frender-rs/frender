use std::borrow::Borrow;

use async_str_iter::borrow_str::IterBorrowStr;
use frender_html::form_control::textarea::{SsrTextAreaValue, TextAreaValue};
use frender_ssr::html::{encode::Encode, escape_safe::Safe};
use hooks::ShareValue;

use super::SignalIntoControlledValue;

impl<S> TextAreaValue for SignalIntoControlledValue<S> {}

impl<S> SsrTextAreaValue for SignalIntoControlledValue<S>
where
    S: ShareValue,
    S::Value: Borrow<str> + 'static + Clone,
{
    type IntoSsrTextAreaValue = Encode<Safe, IterBorrowStr<S::Value>>;

    fn into_ssr_text_area_value(self) -> Self::IntoSsrTextAreaValue {
        Encode::new(Safe, IterBorrowStr::new(self.0.map(Clone::clone)))
    }
}

#[cfg(feature = "csr")]
mod csr {
    use frender_html::form_control::{
        textarea::CsrTextAreaValue, KindOfValue, ProvideFormControlValue,
    };
    use hooks::Signal;

    use crate::hooks_ext::form_control::{FromFormControlValue, SignalIntoControlledValue};

    impl<S: Signal + 'static> CsrTextAreaValue for SignalIntoControlledValue<S>
    where
        S::SignalHook: Unpin,
        S::Value: FromFormControlValue<KindOfValue> + ProvideFormControlValue<KindOfValue>,
    {
        type IntoCsrTextAreaValue = Self; // TODO: into instead of self

        fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
            self
        }
    }
}
