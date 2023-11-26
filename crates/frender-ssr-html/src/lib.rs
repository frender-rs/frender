pub mod assert;
pub mod attr;
pub mod attr_value;
pub mod encode;
pub mod escape_safe;

pub(crate) mod sealed {
    pub trait Sealed {}
}
