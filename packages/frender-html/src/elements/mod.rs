pub mod array;
pub mod boxed;
pub mod either;
pub mod option;
pub mod reactive_value;
pub mod tuple;

pub use crate::intrinsic::csr as intrinsic;

mod empty;

enum Never {}

pub(crate) struct Kind<K: ?Sized>(std::marker::PhantomData<K>, Never);

macro_rules! unreachable_debug {
    ($($args:tt)+) => {{
        #[cfg(debug_assertions)]
        unreachable!($($args)+);
        #[cfg(not(debug_assertions))]
        unreachable!();
    }};
}

use unreachable_debug;
