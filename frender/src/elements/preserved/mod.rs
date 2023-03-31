pub struct Preserved<E>(pub E);

#[cfg(feature = "either")]
pub mod either;
pub mod option;
