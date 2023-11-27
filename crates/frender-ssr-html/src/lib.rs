pub mod assert;
pub mod attr;
pub mod attr_value;
pub mod element;
pub mod encode;
pub mod escape_safe;
pub mod scalar;
pub mod tag;

pub(crate) mod sealed {
    pub trait Sealed {}
}

pub(crate) mod utils;
