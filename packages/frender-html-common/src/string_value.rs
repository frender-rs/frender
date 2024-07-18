use async_str_iter::IntoAsyncStrIterator;

pub trait StringValue:
    AsRef<str> + IntoAsyncStrIterator<IntoAsyncStrIterator = Self::OneString>
{
    type OneString: frender_ssr_html::assert::OneString;
}

frender_common::impl_many!(
    impl<__> StringValue
        for each_of![
            async_str_iter::never::Never,
            &str,
            String,
            std::borrow::Cow<'_, str>,
            std::rc::Rc<str>,
            std::sync::Arc<str>,
        ]
    {
        type OneString = <Self as IntoAsyncStrIterator>::IntoAsyncStrIterator;
    }
);

pub trait MaybeStringValue {
    type StringValue: StringValue;

    fn maybe_string_value(this: Self) -> Option<Self::StringValue>;
}

impl<S: StringValue> MaybeStringValue for S {
    type StringValue = S;

    fn maybe_string_value(this: Self) -> Option<Self::StringValue> {
        Some(this)
    }
}

impl<T: MaybeStringValue> MaybeStringValue for Option<T> {
    type StringValue = T::StringValue;

    fn maybe_string_value(this: Self) -> Option<Self::StringValue> {
        this.and_then(T::maybe_string_value)
    }
}

pub enum EitherStringValue<L, R> {
    Left(L),
    Right(R),
}

impl<L: IntoAsyncStrIterator, R: IntoAsyncStrIterator> IntoAsyncStrIterator
    for EitherStringValue<L, R>
{
    type IntoAsyncStrIterator =
        async_str_iter::either::IterEither<L::IntoAsyncStrIterator, R::IntoAsyncStrIterator>;

    fn into_async_str_iterator(self) -> Self::IntoAsyncStrIterator {
        use async_str_iter::either::IterEither;
        match self {
            EitherStringValue::Left(this) => IterEither::Left(this.into_async_str_iterator()),
            EitherStringValue::Right(this) => IterEither::Right(this.into_async_str_iterator()),
        }
    }
}

impl<L: AsRef<str>, R: AsRef<str>> AsRef<str> for EitherStringValue<L, R> {
    fn as_ref(&self) -> &str {
        match self {
            EitherStringValue::Left(this) => this.as_ref(),
            EitherStringValue::Right(this) => this.as_ref(),
        }
    }
}

impl<L: StringValue, R: StringValue> StringValue for EitherStringValue<L, R> {
    type OneString = async_str_iter::either::IterEither<L::OneString, R::OneString>;
}

#[cfg(feature = "either")]
impl<L: MaybeStringValue, R: MaybeStringValue> MaybeStringValue for either::Either<L, R> {
    type StringValue = EitherStringValue<L::StringValue, R::StringValue>;

    fn maybe_string_value(this: Self) -> Option<Self::StringValue> {
        match this {
            either::Either::Left(this) => L::maybe_string_value(this).map(EitherStringValue::Left),
            either::Either::Right(this) => {
                R::maybe_string_value(this).map(EitherStringValue::Right)
            }
        }
    }
}
