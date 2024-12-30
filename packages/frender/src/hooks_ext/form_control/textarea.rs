use std::borrow::Borrow;

use async_str_iter::borrow_str::IterBorrowStr;
use frender_html::form_control::textarea::SsrTextAreaValue;
use frender_ssr::html::{encode::Encode, escape_safe::Safe};
use hooks::ShareValue;

use super::SignalIntoControlledValue;

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
