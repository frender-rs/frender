pub mod input;
pub mod textarea;

mod known_str;

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

#[cfg(feature = "ssr")]
pub mod ssr {
    pub(crate) mod value;
}
