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

mod sealed {
    pub trait Tuple {}
}

#[derive(Debug, Clone, Copy)]
pub struct HkFn<F>(F);

#[cfg(feature = "impl_with_macro_rules")]
mod imp_macros;

#[cfg(feature = "impl_with_macro_rules")]
mod imp {
    super::imp_macros::impl_with_macro_rules!(a1: A1, a2: A2, a3: A3, a4: A4, a5: A5);
}

#[cfg(not(feature = "impl_with_macro_rules"))]
mod imp;

pub use imp::*;

#[cfg(test)]
mod tests {
    use super::{Callable, CallableWithFixedArguments, Callback};

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
