mod array;
mod either;
mod empty;
mod option;
mod prefix_cursor_placeholder;
mod reactive_value;
mod tuple;

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
