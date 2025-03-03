use std::borrow::{Borrow, Cow};

use async_str_iter::{AsyncStrIterator, IntoAsyncStrIterator};

use crate::{
    non_reactive::{Uncached, UncachedNonReactiveValue},
    static_or_temp_ref::StaticOrTempRef,
    temp_into_static::{IntoStatic, TempIntoStatic},
    temp_ref::TempRef,
    value_kind::KindOfTempRef,
};

pub trait SsrStr {
    type SsrStrIntoAsyncStrIterator: AsyncStrIterator;
    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator;
}

impl<T: 'static + Borrow<str> + IntoAsyncStrIterator> SsrStr for T {
    type SsrStrIntoAsyncStrIterator = T::IntoAsyncStrIterator;

    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator {
        self.into_async_str_iterator()
    }
}

impl SsrStr for StaticOrTempRef<'_, str> {
    type SsrStrIntoAsyncStrIterator =
        <Cow<'static, str> as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator {
        self.to_owned_cow_static().into_async_str_iterator()
    }
}

impl<T: IntoStatic<str>> SsrStr for TempIntoStatic<T> {
    type SsrStrIntoAsyncStrIterator = async_str_iter::borrow_str::IterBorrowStr<T::IntoStatic>;

    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator {
        Self::SsrStrIntoAsyncStrIterator::new(self.0.into_static())
    }
}

impl<'a> SsrStr for TempRef<'a, str> {
    type SsrStrIntoAsyncStrIterator = <&'a str as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator {
        self.0.into_async_str_iterator()
    }
}

/// [`Uncached`] derives [`SsrStr`].
impl<T: UncachedNonReactiveValue<KindOfTempRef<str>> + SsrStr> SsrStr for Uncached<T> {
    type SsrStrIntoAsyncStrIterator = T::SsrStrIntoAsyncStrIterator;

    fn ssr_str_into_async_str_iterator(self) -> Self::SsrStrIntoAsyncStrIterator {
        self.0.ssr_str_into_async_str_iterator()
    }
}
