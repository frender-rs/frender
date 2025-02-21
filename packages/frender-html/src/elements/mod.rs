#[cfg(feature = "csr")]
mod array;
#[cfg(feature = "csr")]
mod either;
#[cfg(feature = "csr")]
mod option;
#[cfg(feature = "csr")] // TODO: ssr
pub mod reactive_value;
#[cfg(feature = "csr")]
mod tuple;

#[cfg(feature = "csr")]
mod empty;

#[cfg(feature = "csr")]
mod prefix_cursor_placeholder;

#[cfg(feature = "csr")]
enum Never {}

#[cfg(feature = "csr")]
pub(crate) struct Kind<K: ?Sized>(std::marker::PhantomData<K>, Never);

#[cfg(feature = "csr")]
macro_rules! unreachable_debug {
    ($($args:tt)+) => {{
        #[cfg(debug_assertions)]
        unreachable!($($args)+);
        #[cfg(not(debug_assertions))]
        unreachable!();
    }};
}

#[cfg(feature = "csr")]
use unreachable_debug;
