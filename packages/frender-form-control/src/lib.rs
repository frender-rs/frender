pub mod input;
pub mod textarea;

mod known;
mod known_provide_str;

mod value;
pub use value::{FormControlValueKind, KindOfChecked, KindOfValue, KindOfValueAsNumber};

pub mod values;

#[cfg(feature = "csr")]
pub mod csr {
    pub(crate) mod element;
    pub use element::FormControlElement;

    pub(crate) mod value;
    pub use value::{FormControlValue, FormControlValueStateKind, HandleFormControlValue};
}

mod provide;
pub use provide::{
    MaybeProvideFormControlValue, ProvideFormControlValue, ProvideFormControlValueWithKind,
};
