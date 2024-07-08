use std::{cell::RefCell, convert::Infallible, thread::LocalKey};

use crate::{ContextKey, ContextKeyInner};

pub type LocalContextKeyProvided<T> = crate::ContextKey<LocalKeyProvided<T>>;

pub struct LocalKeyProvided<T: 'static>(LocalKey<RefCell<T>>);

impl<T: 'static> LocalKeyProvided<T> {
    pub const fn new(inner: LocalKey<RefCell<T>>) -> Self {
        Self(inner)
    }
}

impl<T: 'static> ContextKeyInner for LocalKeyProvided<T> {
    type Value = T;
    type Error = Infallible;

    fn try_get(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Copy,
    {
        Ok(self.get())
    }

    fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.0.with_borrow(|v| *v)
    }

    fn try_get_cloned(&'static self) -> Result<Self::Value, Self::Error>
    where
        Self::Value: Clone,
    {
        Ok(self.get_cloned())
    }

    fn get_cloned(&'static self) -> T
    where
        T: Clone,
    {
        self.0.with_borrow(T::clone)
    }

    fn try_map<R>(&'static self, f: impl FnOnce(&Self::Value) -> R) -> Result<R, Self::Error> {
        Ok(self.map(f))
    }

    fn map<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        self.0.with_borrow(f)
    }

    fn is_same_as(&'static self, other: &'static Self) -> bool {
        std::ptr::eq(&self.0, &other.0)
    }

    type SwapValue = T;

    fn swap_value(&'static self, value: &mut Self::SwapValue) {
        self.0
            .with_borrow_mut(|context| std::mem::swap(context, value))
    }

    #[inline(always)]
    fn make_swap_value(value: Self::Value) -> Self::SwapValue {
        value
    }

    type MaybeContextKeyAndValue = Option<(&'static ContextKey<Self>, T)>;
}

#[cfg(test)]
mod tests {
    local_context!(
        static CTX_U8: u8 = const { 1 };
    );

    local_context!(
        static CTX_STRING: String = "hello".to_string();
    );

    #[test]
    fn unprovided() {
        assert_eq!(CTX_U8.get(), 1);
        assert_eq!(CTX_STRING.get_cloned(), "hello");
    }

    #[test]
    fn provided() {
        {
            let mut value = 2;
            let res = CTX_U8.provide_value(&mut value, || CTX_U8.get());
            assert_eq!(value, 2);
            assert_eq!(res, 2);
            assert_eq!(CTX_U8.try_get().unwrap(), 1);
        }

        {
            let mut value = "world".to_string();
            let res = CTX_STRING.provide_value(&mut value, || CTX_STRING.get_cloned());
            assert_eq!(value, "world");
            assert_eq!(res, "world");
            assert_eq!(CTX_STRING.try_get_cloned().unwrap(), "hello");
        }
    }
}
