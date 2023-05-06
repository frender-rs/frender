//!
//! ```
//! # use frender_events::*;
//! let callback = callback::r#ref(u8::clone).with_input_ref(8);
//! assert_eq!(callback.emit(()), 8u8);
//! ```

mod traits;
pub use traits::*;

pub mod accept_anything;
pub mod chain;

pub mod argument;

mod maybe_handle_event;
pub use maybe_handle_event::*;

mod sealed {
    pub trait Tuple {}
}

#[derive(Debug, Clone, Copy)]
pub struct HkFn<F>(F);

#[cfg(feature = "impl_with_macro_rules")]
mod imp_macros;

#[cfg(feature = "impl_with_macro_rules")]
mod imp {
    super::imp_macros::impl_with_macro_rules!(a1: A1, a2: A2, a3: A3);
}

#[cfg(not(feature = "impl_with_macro_rules"))]
mod imp;

pub use imp::*;

#[macro_export]
macro_rules! ArgumentTypes {
    () => {
        ()
    };
    (@$resolved:tt) => {
        $resolved
    };
    ($(@($($resolved:tt)*))? &mut $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::callback::argument::ByMut<$t>,) $($rest)* }
    };
    ($(@($($resolved:tt)*))? &    $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::callback::argument::ByRef<$t>,) $($rest)* }
    };
    ($(@($($resolved:tt)*))?      $t:ty $(, $($rest:tt)*)? ) => {
        $crate::ArgumentTypes! { @($($($resolved)*)? $crate::callback::argument::Value<$t>,) $($rest)* }
    };
}

#[cfg(test)]
mod tests {
    use super::{Callable, Callback, IsCallable};

    #[test]
    fn test_callback_ref() {
        fn asserts<T>(t: T) -> T
        where
            T: for<'i> Callback<&'i usize, Output = usize>,
        {
            t
        }

        let cbk = asserts(crate::callback::r#ref(Clone::clone));
        assert_eq!(cbk.emit(&0), 0);
        assert_eq!(cbk.provide_last_argument_refed(8).call_fn(()), 8);
    }

    #[test]
    fn mods() {
        use super::super::callback;
        let _: fn() = callback(|| {});

        let _: fn(()) = callback::value(|()| {});
        let _: callback::HkFn<fn(&())> = callback::r#ref(|&()| {});
        let _: callback::HkFn<fn(&mut ())> = callback::r#mut(|&mut ()| {});

        let _: fn((), ()) = callback::value::value(|(), ()| {});
        let _: callback::value::HkFn<fn((), &())> = callback::value::r#ref(|(), &()| {});
    }
}
