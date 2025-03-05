use frender_form_control::textarea::TextAreaValue;

use super::SignalIntoControlledValue;

impl<S> TextAreaValue for SignalIntoControlledValue<S> {}

#[cfg(feature = "ssr")]
mod ssr {
    use async_str_iter::borrow_str::IterBorrowStr;

    use frender_form_control::textarea::SsrTextAreaValue;
    use frender_ssr::html::escape_safe::Safe;

    use frender_ssr::html::encode::Encode;

    use std::borrow::Borrow;

    use hooks::ShareValue;

    use crate::share_value::form_control::SignalIntoControlledValue;

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
}

#[cfg(feature = "csr")]
mod csr {
    use frender_form_control::{KindOfValue, textarea::CsrTextAreaValue};
    use hooks::Signal;

    use crate::share_value::form_control::{
        FromFormControlValue, SignalIntoControlledValue, ToProvideFormControlValue,
    };

    impl<S: Signal + 'static> CsrTextAreaValue for SignalIntoControlledValue<S>
    where
        S::SignalHook: Unpin,
        S::Value: FromFormControlValue<KindOfValue> + ToProvideFormControlValue<KindOfValue>,
    {
        type IntoCsrTextAreaValue = Self; // TODO: into instead of self

        fn into_csr_text_area_value(self) -> Self::IntoCsrTextAreaValue {
            self
        }
    }
}
