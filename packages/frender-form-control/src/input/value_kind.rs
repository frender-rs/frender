#[cfg(feature = "csr")]
pub use self::csr::InputValueKindCsr;
#[cfg(feature = "ssr")]
pub use self::ssr::InputValueKindSsr;

#[cfg(feature = "ssr")]
use std::convert::identity;

use crate::{
    value::{KindOfValue, KindOfValueAsNumber},
    FormControlValueKind,
};

#[cfg(feature = "ssr")]
use crate::ProvideFormControlValue;

#[cfg(feature = "csr")]
use super::InputElement;

#[cfg(feature = "csr")]
mod csr;
#[cfg(feature = "ssr")]
mod ssr;

mod sealed {
    pub trait InputValueKind {}
}

#[cfg(not(feature = "csr"))]
#[cfg(not(feature = "ssr"))]
pub trait InputValueKind: sealed::InputValueKind + FormControlValueKind {}

#[cfg(feature = "csr")]
#[cfg(not(feature = "ssr"))]
pub trait InputValueKind:
    sealed::InputValueKind + FormControlValueKind + InputValueKindCsr
{
}

#[cfg(feature = "ssr")]
#[cfg(not(feature = "csr"))]
pub trait InputValueKind:
    sealed::InputValueKind + FormControlValueKind + InputValueKindSsr
{
}

/// Types that can be used as `input.value`.
#[cfg(feature = "csr")]
#[cfg(feature = "ssr")]
pub trait InputValueKind:
    sealed::InputValueKind + FormControlValueKind + InputValueKindCsr + InputValueKindSsr
{
}

macro_rules! as_mut_form_control_element {
    () => {
        type AsMutFormControlElement<E: ?Sized + InputElement<R>, R: ?Sized> = E;
        fn as_mut_form_control_element<E: ?Sized + InputElement<R>, R: ?Sized>(
            el: &mut E,
        ) -> &mut Self::AsMutFormControlElement<E, R> {
            el
        }
    };
}

// region: input.value
impl sealed::InputValueKind for KindOfValue {}
impl InputValueKind for KindOfValue {}
#[cfg(feature = "ssr")]
impl InputValueKindSsr for KindOfValue {
    // TODO: optimize
    type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>> =
        frender_ssr::html::attr_value::AttrEqValue<
            async_str_iter::borrow_str::IterBorrowStr<String>,
        >;

    fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(
        v: V,
        _: &str,
    ) -> Self::IntoInputValueAttrValue<V> {
        let v = v.provide_form_control_value(str::to_owned);
        frender_ssr::html::attr_value::AttrEqValue(async_str_iter::borrow_str::IterBorrowStr::new(
            v,
        ))
    }
}
#[cfg(feature = "csr")]
impl InputValueKindCsr for KindOfValue {
    as_mut_form_control_element! {}
}
// endregion
// region: input.valueAsNumber
mod convert_number_to_string;
impl sealed::InputValueKind for KindOfValueAsNumber {}
impl InputValueKind for KindOfValueAsNumber {}
#[cfg(feature = "ssr")]
impl InputValueKindSsr for KindOfValueAsNumber {
    type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>> =
        frender_ssr::html::attr_value::AttrEqValue<
            <String as async_str_iter::IntoAsyncStrIterator>::IntoAsyncStrIterator,
        >;

    fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(
        v: V,
        input_type: &str,
    ) -> Self::IntoInputValueAttrValue<V> {
        let value = v.provide_form_control_value(identity);
        let value = convert_number_to_string::convert_number_to_string(input_type, value);
        frender_ssr::html::attr_value::AttrEqValue::new(
            async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(value),
        )
    }
}
#[cfg(feature = "csr")]
impl InputValueKindCsr for KindOfValueAsNumber {
    as_mut_form_control_element! {}
}
// endregion
