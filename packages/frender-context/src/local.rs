use crate::ContextValueNotProvided;

pub use self::provided::{LocalContextKeyProvided, LocalKeyProvided};

use std::{cell::RefCell, thread::LocalKey};

#[macro_export]
macro_rules! local_context {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => {
        $(#[$attr])*
        $vis const $name: $crate::local::LocalContextKeyUnprovided<$t> = {
            $crate::local::__private::thread_local! {
                static CTX: $crate::local::__private::RefCellUnprovided<$t> = const { $crate::local::__private::ref_cell_unprovided() };
            }

            $crate::ContextKey::new($crate::local::LocalKeyUnprovided::new(CTX))
        };
        $crate::local_context! { $($rest)* }
    };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = const $init:block; $($rest:tt)*) => {
        $(#[$attr])*
        $vis const $name: $crate::local::LocalContextKeyProvided<$t> = {
            const CONTEXT_VALUE_INIT_EXPR: $t = $init;
            $crate::local::__private::thread_local! {
                static CTX: $crate::local::__private::RefCellProvided<$t> = const { $crate::local::__private::RefCellProvided::new(CONTEXT_VALUE_INIT_EXPR) };
            }

            $crate::ContextKey::new($crate::local::LocalKeyProvided::new(CTX))
        };
        $crate::local_context! { $($rest)* }
    };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty = $init:expr; $($rest:tt)*) => {
        $(#[$attr])*
        $vis const $name: $crate::local::LocalContextKeyProvided<$t> = {
            $crate::local::__private::thread_local! {
                static CTX: $crate::local::__private::RefCellProvided<$t> = $crate::local::__private::RefCellProvided::new($init);
            }

            $crate::ContextKey::new($crate::local::LocalKeyProvided::new(CTX))
        };
        $crate::local_context! { $($rest)* }
    };
}

mod provided;

pub type LocalContextKeyUnprovided<T> = crate::ContextKey<LocalKeyUnprovided<T>>;

pub struct LocalKeyUnprovided<T: 'static>(LocalKey<RefCell<Result<T, ContextValueNotProvided>>>);

impl<T: 'static> LocalKeyUnprovided<T> {
    pub const fn new(inner: LocalKey<RefCell<Result<T, ContextValueNotProvided>>>) -> Self {
        Self(inner)
    }
}

impl<T> crate::ContextKeyInner for LocalKeyUnprovided<T> {
    type Value = T;
    type Error = ContextValueNotProvided;

    fn try_get(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Copy,
    {
        self.0.with_borrow(|v| *v)
    }

    fn try_get_cloned(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Clone,
    {
        self.0.with_borrow(Clone::clone)
    }

    fn try_map<R>(&'static self, f: impl FnOnce(&Self::Value) -> R) -> Result<R, Self::Error> {
        self.0.with_borrow(|v| match v {
            Ok(v) => Ok(f(v)),
            Err(error) => Err(*error),
        })
    }

    fn is_same_as(&'static self, other: &'static Self) -> bool {
        std::ptr::eq(&self.0, &other.0)
    }

    type SwapValue = Result<T, Self::Error>;

    fn swap_value(&'static self, value: &mut Self::SwapValue) {
        self.0
            .with_borrow_mut(|context| std::mem::swap(context, value));
    }

    fn make_swap_value(value: Self::Value) -> Self::SwapValue {
        Ok(value)
    }

    type MaybeContextKeyAndValue = crate::MaybeContextKeyAndValueUnprovided<Self>;
}

#[doc(hidden)]
pub mod __private {
    pub use std::thread_local;

    use crate::ContextValueNotProvided;

    pub type RefCellUnprovided<T> = std::cell::RefCell<Result<T, ContextValueNotProvided>>;
    pub const fn ref_cell_unprovided<T>() -> RefCellUnprovided<T> {
        RefCellUnprovided::new(Err(ContextValueNotProvided))
    }
    pub type RefCellProvided<T> = std::cell::RefCell<T>;
}

#[cfg(test)]
mod tests {
    // ElementWithContext will update context_key and value if the address differs from the address in the render state.
    #[test]
    fn same_address() {
        use std::thread::LocalKey;
        thread_local!(
            static VALUE: u8 = const { 0 };
        );

        const VALUE_2: LocalKey<u8> = VALUE;

        assert!(std::ptr::eq(&VALUE, &VALUE));
        assert!(std::ptr::eq(&VALUE, &VALUE_2));

        fn get_ref() -> &'static LocalKey<u8> {
            &VALUE
        }

        fn get_ref_2() -> &'static LocalKey<u8> {
            &VALUE
        }

        assert!(std::ptr::eq(get_ref(), get_ref_2()));

        fn get_ref_conditional(condition: bool) -> &'static LocalKey<u8> {
            if condition {
                &VALUE
            } else {
                &VALUE
            }
        }

        assert!(std::ptr::eq(
            get_ref_conditional(false),
            get_ref_conditional(true)
        ));
    }

    local_context!(
        static CTX_U8: u8;
    );

    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: ContextValueNotProvided"
    )]
    fn unprovided() {
        CTX_U8.get();
    }

    #[test]
    fn provided() {
        let res = CTX_U8.provide_value(&mut Ok(2), || CTX_U8.get());
        assert_eq!(res, 2);
        assert!(CTX_U8.try_get().is_err());
    }
}
