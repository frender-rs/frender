pub trait FormControlValueKind: 'static {
    /// The value passed to formControl.value api
    type Value<'a>;

    /// The value, reference, or `Cow` passed on change.
    type FormControlValue<'a>;
}

pub enum KindOfValue {}

impl FormControlValueKind for KindOfValue {
    type Value<'a> = &'a str;
    type FormControlValue<'a> = std::borrow::Cow<'a, str>;
}

pub enum KindOfChecked {}

impl FormControlValueKind for KindOfChecked {
    type Value<'a> = bool;
    type FormControlValue<'a> = bool;
}

pub enum KindOfValueAsNumber {}

impl FormControlValueKind for KindOfValueAsNumber {
    type Value<'a> = f64;
    type FormControlValue<'a> = f64;
}
