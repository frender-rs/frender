pub mod input;
pub mod textarea;

mod known_str;

mod value;
pub use value::FormControlValueKind;

pub mod values;

#[cfg(feature = "csr")]
pub mod csr {
    pub(crate) mod element;
    pub(crate) mod value;
}

#[cfg(feature = "ssr")]
pub mod ssr {
    pub(crate) mod value;
}
