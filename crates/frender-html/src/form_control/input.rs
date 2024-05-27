pub use value_kind::InputValueKind;

use async_str_iter::IntoAsyncStrIterator;
use frender_common::PrimarilyBorrow;
use frender_html_common::MaybeStringValue;

use crate::form_control::value::{FormControlValue, FormControlValueKind, UncontrolledWithDefaultValue};

use super::{
    element::FormControlElement,
    value::{MaybeProvideFormControlValue, ProvideFormControlValue},
};

/// A trait alias
pub trait InputElement<Renderer: ?Sized>: FormControlElement<str, Renderer> + FormControlElement<bool, Renderer> + FormControlElement<f64, Renderer> {}

impl<E, Renderer: ?Sized> InputElement<Renderer> for E where E: FormControlElement<str, Renderer> + FormControlElement<bool, Renderer> + FormControlElement<f64, Renderer> {}

mod value_kind {
    use frender_common::convert::{FromMut, IntoMut};

    use crate::form_control::{
        element::FormControlElement,
        value::{FormControlValueKind, ProvideFormControlValue},
    };

    use super::InputElement;

    pub trait InputValueKind: FormControlValueKind {
        type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>>: frender_ssr::html::assert::HtmlAttributeEqValueOrEmpty;
        fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(v: V, input_type: &str) -> Self::IntoInputValueAttrValue<V>;

        type AsMutFormControlElement<E: ?Sized + InputElement<R>, R: ?Sized>: ?Sized + FormControlElement<Self, R> + FromMut<E> + IntoMut<E>;
        fn as_mut_form_control_element<E: ?Sized + InputElement<R>, R: ?Sized>(el: &mut E) -> &mut Self::AsMutFormControlElement<E, R>;
    }

    macro_rules! as_mut_form_control_element {
        () => {
            type AsMutFormControlElement<E: ?Sized + InputElement<R>, R: ?Sized> = E;
            fn as_mut_form_control_element<E: ?Sized + InputElement<R>, R: ?Sized>(el: &mut E) -> &mut Self::AsMutFormControlElement<E, R> {
                el
            }
        };
    }

    impl InputValueKind for str {
        // TODO: optimize
        type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>> = frender_ssr::html::attr_value::AttrEqValue<async_str_iter::borrow_str::IterBorrowStr<String>>;

        fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(v: V, _: &str) -> Self::IntoInputValueAttrValue<V> {
            let v = v.provide_form_control_value(str::to_owned);
            frender_ssr::html::attr_value::AttrEqValue(async_str_iter::borrow_str::IterBorrowStr::new(v))
        }

        as_mut_form_control_element! {}
    }

    impl InputValueKind for f64 {
        type IntoInputValueAttrValue<V: ProvideFormControlValue<Self>> = frender_ssr::html::attr_value::AttrEqValue<<String as async_str_iter::IntoAsyncStrIterator>::IntoAsyncStrIterator>;

        fn into_input_value_attr_value<V: ProvideFormControlValue<Self>>(v: V, input_type: &str) -> Self::IntoInputValueAttrValue<V> {
            let value = v.provide_form_control_value(|v| *v);
            let value = super::convert_number_to_string(input_type, value);
            frender_ssr::html::attr_value::AttrEqValue::new(async_str_iter::IntoAsyncStrIterator::into_async_str_iterator(value))
        }

        as_mut_form_control_element! {}
    }
}

#[cfg(feature = "web")]
pub(crate) mod web {
    use wasm_bindgen::prelude::*;

    // https://html.spec.whatwg.org/multipage/input.html#concept-input-value-number-string
    #[wasm_bindgen(module = "/js/input.js")]
    extern "C" {
        #[wasm_bindgen(js_name = "numberAsInputValue")]
        pub(crate) fn number_as_input_value(input_type: &str, value: f64) -> String;

        #[wasm_bindgen(js_name = "setDefaultValueAsNumber")]
        pub(crate) fn set_default_value(input: &web_sys::HtmlInputElement, defaultValue: f64);
    }
}

#[cfg(feature = "chrono")]
#[cfg(not(all(feature = "web", target_arch = "wasm32")))]
mod convert_number_to_string_with_chrono {
    use chrono::prelude::*;

    fn millis_to_secs_and_nsecs(millis: f64) -> (i64, u32) {
        let secs = millis.div_euclid(1000.0) as i64;
        let nsecs = (millis.rem_euclid(1000.0) * 1_000_000.0) as u32;
        (secs, nsecs)
    }

    fn millis_to_date_time(millis: f64) -> Option<DateTime<Utc>> {
        let (secs, nsecs) = millis_to_secs_and_nsecs(millis);
        DateTime::from_timestamp(secs, nsecs)
    }

    fn millis_to_date(millis: f64) -> Option<NaiveDate> {
        millis_to_date_time(millis).map(|dt| dt.date_naive())
    }

    fn month_to_date(month_num: i32) -> Option<NaiveDate> {
        const DATE_UNIX_EPOCH: NaiveDate = NaiveDateTime::UNIX_EPOCH.date();
        let months = chrono::Months::new(month_num.unsigned_abs());
        if month_num < 0 {
            DATE_UNIX_EPOCH.checked_sub_months(months)
        } else {
            DATE_UNIX_EPOCH.checked_add_months(months)
        }
    }

    // TODO: this might have leading `-` or `+` but input value doesn't allow that.
    fn date_string(dt: NaiveDate) -> String {
        dt.to_string()
    }

    fn month_string(dt: NaiveDate) -> String {
        let mut s = date_string(dt);
        let index_of_last_hyphen = {
            let s = s.as_bytes();
            let mut i = s.len() - 1;
            while s[i] != b'-' {
                i -= 1;
            }

            i
        };
        s.truncate(index_of_last_hyphen);

        s
    }

    fn week_string(dt: NaiveDate) -> String {
        let w = dt.iso_week();
        format!("{:?}", w)
    }

    fn millis_to_time(millis: f64) -> Option<NaiveTime> {
        let (secs, nsecs) = millis_to_secs_and_nsecs(millis);

        let secs = secs.rem_euclid(86_400);

        NaiveTime::from_num_seconds_from_midnight_opt(secs as u32, nsecs)
    }

    fn time_string(t: NaiveTime) -> String {
        t.to_string()
    }

    fn datetime_string(dt: DateTime<Utc>) -> String {
        format!("{:?}", dt.naive_utc())
    }

    pub(super) fn convert_non_nan_number_to_string(input_type: &str, value: f64) -> String {
        match input_type {
            "date" => millis_to_date(value).map(date_string),
            "month" => month_to_date(value as i32).map(month_string),
            "week" => millis_to_date(value).map(week_string),
            "time" => millis_to_time(value).map(time_string),
            "datetime-local" => millis_to_date_time(value).map(datetime_string),
            "number" | "range" => return value.to_string(), // TODO: is this compatible with js implementation?
            _ => None,
        }
        .unwrap_or_default()
    }
}

// https://html.spec.whatwg.org/multipage/input.html#concept-input-value-number-string
fn convert_number_to_string(input_type: &str, value: f64) -> String {
    if value.is_nan() {
        return String::new();
    }

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    return self::web::number_as_input_value(input_type, value);

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    #[cfg(feature = "chrono")]
    return convert_number_to_string_with_chrono::convert_non_nan_number_to_string(input_type, value);

    #[cfg(not(all(feature = "web", target_arch = "wasm32")))]
    #[cfg(not(feature = "chrono"))]
    return match input_type {
        "number" | "range" => value.to_string(),
        _ => String::new(),
    };
}

pub trait InputValue: FormControlValue<Self::ValueKind> + MaybeProvideFormControlValue<Self::ValueKind> {
    type ValueKind: ?Sized + FormControlValueKind + InputValueKind;
}

/// A trait alias for [`FormControlValue<bool>`] + [`MaybeProvideFormControlValue<bool>`].
pub trait InputChecked: FormControlValue<bool> + MaybeProvideFormControlValue<bool> {}

impl<T: FormControlValue<bool> + MaybeProvideFormControlValue<bool>> InputChecked for T {}

// Uncontrolled
impl InputValue for () {
    type ValueKind = str;
}

// Uncontrolled number or date/time with default value
impl InputValue for f64 {
    type ValueKind = f64;
}

// Uncontrolled string with default value
frender_common::impl_many!(
    impl<__> InputValue
        for each_of![
            //
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>
        ]
    {
        type ValueKind = str;
    }
);

impl<V: PrimarilyBorrow<Borrowed = VK> + PartialEq, VK: ?Sized + FormControlValueKind + InputValueKind> InputValue for UncontrolledWithDefaultValue<V> {
    type ValueKind = VK;
}

#[cfg(feature = "either")]
impl<L: InputValue, R: InputValue<ValueKind = L::ValueKind>> InputValue for either::Either<L, R> {
    type ValueKind = L::ValueKind;
}

pub struct InputDataModel<Type: MaybeStringValue, Value: InputValue, Checked: InputChecked> {
    pub r#type: Type,
    pub value: Value,
    pub checked: Checked,
}

impl<Type: MaybeStringValue, Value: InputValue, Checked: InputChecked> InputDataModel<Type, Value, Checked> {
    pub fn map_type<V: MaybeStringValue>(self, f: impl FnOnce(Type) -> V) -> InputDataModel<V, Value, Checked> {
        let Self { r#type, value, checked } = self;
        InputDataModel { r#type: f(r#type), value, checked }
    }

    pub fn map_value<V: InputValue>(self, f: impl FnOnce(Value) -> V) -> InputDataModel<Type, V, Checked> {
        let Self { r#type, value, checked } = self;
        InputDataModel { r#type, value: f(value), checked }
    }

    pub fn map_checked<V: InputChecked>(self, f: impl FnOnce(Checked) -> V) -> InputDataModel<Type, Value, V> {
        let Self { r#type, value, checked } = self;
        InputDataModel { r#type, value, checked: f(checked) }
    }
}

pub trait IntoInputDataModel {
    type Type: MaybeStringValue;
    type Value: InputValue;
    type Checked: InputChecked;

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked>;
}

impl IntoInputDataModel for () {
    type Type = ();
    type Value = ();
    type Checked = ();

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked> {
        InputDataModel { r#type: (), value: (), checked: () }
    }
}

impl<
        //
        Type: MaybeStringValue,
        Value: InputValue,
        Checked: InputChecked,
    > IntoInputDataModel for InputDataModel<Type, Value, Checked>
{
    type Type = Type;
    type Value = Value;
    type Checked = Checked;

    fn into_input_data_model(self) -> InputDataModel<Self::Type, Self::Value, Self::Checked> {
        self
    }
}

mod ssr {
    use async_str_iter::{chain::Chain, option::IterOption};
    use frender_dom::component::IntoSpaceAndHtmlAttributesOrEmpty;
    use frender_html_common::StringValue;
    use frender_ssr::html::{
        attr::{AssertSpaceAndHtmlAttributeName, SpaceAndHtmlAttribute},
        attr_value::AttrEqValue,
    };

    use super::*;

    impl<
            //
            Type: MaybeStringValue,
            Value: InputValue,
            Checked: InputChecked,
        > IntoSpaceAndHtmlAttributesOrEmpty for InputDataModel<Type, Value, Checked>
    {
        type SpaceAndHtmlAttributesOrEmpty = Chain<
            // type
            IterOption<
                SpaceAndHtmlAttribute<
                    //
                    AssertSpaceAndHtmlAttributeName<&'static str>,
                    AttrEqValue<<<Type as MaybeStringValue>::StringValue as StringValue>::OneString>,
                >,
            >,
            Chain<
                // value
                IterOption<
                    SpaceAndHtmlAttribute<
                        //
                        AssertSpaceAndHtmlAttributeName<&'static str>,
                        <Value::ValueKind as InputValueKind>::IntoInputValueAttrValue<
                            //
                            <Value as MaybeProvideFormControlValue<Value::ValueKind>>::ProvideFormControlValue,
                        >,
                    >,
                >,
                // checked
                IterOption<AssertSpaceAndHtmlAttributeName<&'static str>>,
            >,
        >;

        fn into_space_and_html_attributes_or_empty(self) -> Self::SpaceAndHtmlAttributesOrEmpty {
            const TYPE: AssertSpaceAndHtmlAttributeName<&'static str> = AssertSpaceAndHtmlAttributeName::new_from_str(" type");
            const VALUE: AssertSpaceAndHtmlAttributeName<&'static str> = AssertSpaceAndHtmlAttributeName::new_from_str(" value");
            const CHECKED: AssertSpaceAndHtmlAttributeName<&'static str> = AssertSpaceAndHtmlAttributeName::new_from_str(" checked");

            let Self { r#type: input_type, value, checked } = self;

            let input_type = Type::maybe_string_value(input_type);

            let value_attr = Value::maybe_into_provide_form_control_value(value)
                .map(|value| {
                    let input_type = input_type.as_ref().map_or("", |v| v.as_ref());
                    Value::ValueKind::into_input_value_attr_value(value, input_type)
                })
                .map(|eq_value| SpaceAndHtmlAttribute(VALUE, eq_value));

            let checked_attr = {
                let checked = Checked::maybe_into_provide_form_control_value(checked).map_or(false, |checked| checked.provide_form_control_value(|v| *v));
                checked.then_some(CHECKED)
            };

            Chain::new(
                input_type
                    .map(|input_type| {
                        let value = input_type.into_async_str_iterator();
                        let value = AttrEqValue(value);
                        SpaceAndHtmlAttribute(TYPE, value)
                    })
                    .into_async_str_iterator(),
                Chain::new(value_attr.into_async_str_iterator(), checked_attr.into_async_str_iterator()),
            )
        }
    }
}
