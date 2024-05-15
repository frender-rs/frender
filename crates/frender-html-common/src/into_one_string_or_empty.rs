use async_str_iter::IntoAsyncStrIterator;
use frender_ssr_html::assert::OneStringOrEmpty;

pub trait IntoOneStringOrEmpty {
    type OneStringOrEmpty: OneStringOrEmpty;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty;
}

impl IntoOneStringOrEmpty for () {
    type OneStringOrEmpty = async_str_iter::empty::Empty;

    fn into_one_string_or_empty((): Self) -> Self::OneStringOrEmpty {
        async_str_iter::empty::Empty
    }
}

impl<T: IntoOneStringOrEmpty> IntoOneStringOrEmpty for Option<T> {
    type OneStringOrEmpty = async_str_iter::option::IterOption<T::OneStringOrEmpty>;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        this.map(T::into_one_string_or_empty)
            .into_async_str_iterator()
    }
}

#[cfg(feature = "either")]
impl<L: IntoOneStringOrEmpty, R: IntoOneStringOrEmpty> IntoOneStringOrEmpty
    for either::Either<L, R>
{
    type OneStringOrEmpty =
        async_str_iter::either::IterEither<L::OneStringOrEmpty, R::OneStringOrEmpty>;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        this.map_either(
            IntoOneStringOrEmpty::into_one_string_or_empty,
            IntoOneStringOrEmpty::into_one_string_or_empty,
        )
        .into_async_str_iterator()
    }
}

impl<S: crate::StringValue> IntoOneStringOrEmpty for S {
    type OneStringOrEmpty = <S as IntoAsyncStrIterator>::IntoAsyncStrIterator;

    fn into_one_string_or_empty(this: Self) -> Self::OneStringOrEmpty {
        this.into_async_str_iterator()
    }
}
