pub struct Preserved<E>(pub E);

mod option;
pub use option::*;

#[cfg(feature = "either")]
mod either;
#[cfg(feature = "either")]
pub use self::either::*;
