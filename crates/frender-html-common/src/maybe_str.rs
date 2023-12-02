use std::borrow::Cow;

use async_str_iter::IntoAsyncStrIterator;
use frender_ssr_html::assert::OneStringOrEmpty;

use crate::MaybeUpdateValueWithState;

pub trait MaybeStr: MaybeUpdateValueWithState<str> {
    type OneStringOrEmpty: OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty;
}

impl MaybeStr for () {
    type OneStringOrEmpty = async_str_iter::empty::Empty;

    fn into_one_string_or_empty((): Self) -> Self::OneStringOrEmpty {
        async_str_iter::empty::Empty
    }
}

impl<T: MaybeStr> MaybeStr for Option<T> {
    type OneStringOrEmpty = async_str_iter::option::IterOption<T::OneStringOrEmpty>;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        this.map(T::into_one_string_or_empty)
            .into_async_str_iterator()
    }
}

frender_common::impl_many!(
    impl<__> MaybeStr for each_of![&str, String, Cow<'_, str>] {
        type OneStringOrEmpty = <Self as IntoAsyncStrIterator>::IntoAsyncStrIterator;

        fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
            this.into_async_str_iterator()
        }
    }
);
