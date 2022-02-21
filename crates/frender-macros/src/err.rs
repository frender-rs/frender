pub trait Combine<V> {
    fn combine_with(self, another: V) -> Self;
}

pub trait OptionCombineExt<V> {
    fn unwrap_combined(self, another: V) -> V;
    fn combine_or_replace(&mut self, another: V) -> &mut Self;
}

impl<V: Combine<V>> OptionCombineExt<V> for Option<V> {
    #[inline]
    fn unwrap_combined(self, another: V) -> V {
        match self {
            Some(v) => v.combine_with(another),
            None => another,
        }
    }

    #[inline]
    fn combine_or_replace(&mut self, another: V) -> &mut Self {
        *self = match self.take() {
            Some(v) => Some(v.combine_with(another)),
            None => Some(another),
        };
        self
    }
}

impl Combine<Self> for darling::Error {
    #[inline]
    fn combine_with(self, another: Self) -> Self {
        Self::multiple(vec![self, another])
    }
}

impl Combine<syn::Error> for darling::Error {
    #[inline]
    fn combine_with(self, another: syn::Error) -> Self {
        Self::multiple(vec![self, another.into()])
    }
}

impl Combine<Self> for syn::Error {
    #[inline]
    fn combine_with(mut self, another: Self) -> Self {
        self.combine(another);
        self
    }
}

#[inline]
pub fn maybe_with_error<T, E>(v: T, error: Option<E>) -> Result<T, (T, E)> {
    if let Some(err) = error {
        Err((v, err))
    } else {
        Ok(v)
    }
}

pub trait RecordError<E> {
    fn record_error(&mut self, error: E) -> &mut Self;
}

pub trait OutputError {
    type OutputError;
    fn output_error(self) -> Option<Self::OutputError>;

    #[inline]
    fn into_value_result<T>(self, v: T) -> Result<T, (T, Self::OutputError)>
    where
        Self: Sized,
    {
        maybe_with_error(v, self.output_error())
    }

    #[inline]
    fn into_option_value_result<T>(self, v: T) -> Result<T, (Option<T>, Self::OutputError)>
    where
        Self: Sized,
    {
        match self.output_error() {
            Some(error) => Err((Some(v), error)),
            None => Ok(v),
        }
    }

    fn record_and_output<E>(mut self, error: E) -> Self::OutputError
    where
        Self: Sized + RecordError<E>,
    {
        self.record_error(error);
        self.output_error().unwrap()
    }
}

impl<E: Combine<E>> RecordError<E> for Option<E> {
    #[inline]
    fn record_error(&mut self, error: E) -> &mut Self {
        self.combine_or_replace(error)
    }
}

impl<E> OutputError for Option<E> {
    type OutputError = E;

    #[inline]
    fn output_error(self) -> Option<Self::OutputError> {
        self
    }
}

impl<E> RecordError<E> for Vec<E> {
    #[inline]
    fn record_error(&mut self, error: E) -> &mut Self {
        self.push(error);
        self
    }
}

impl RecordError<syn::Error> for Vec<darling::Error> {
    #[inline]
    fn record_error(&mut self, error: syn::Error) -> &mut Self {
        self.push(error.into());
        self
    }
}

impl OutputError for Vec<darling::Error> {
    type OutputError = darling::Error;

    #[inline]
    fn output_error(self) -> Option<Self::OutputError> {
        if self.len() == 0 {
            None
        } else {
            Some(darling::Error::multiple(self))
        }
    }
}

pub type ValueResult<T, E> = Result<T, (T, E)>;

pub trait ResultUnwrapValueAndErrorExt {
    type Value;
    type Error;
    fn unwrap_value_and_error(self) -> (Self::Value, Option<Self::Error>);

    #[inline]
    fn unwrap_value_and_record_error<R: RecordError<Self::Error>>(
        self,
        recorder: &mut R,
    ) -> Result<Self::Value, Self::Value>
    where
        Self: Sized,
    {
        let (v, error) = self.unwrap_value_and_error();
        if let Some(error) = error {
            recorder.record_error(error);
            Err(v)
        } else {
            Ok(v)
        }
    }
}

impl<V, E> ResultUnwrapValueAndErrorExt for ValueResult<V, E> {
    type Value = V;
    type Error = E;

    #[inline]
    fn unwrap_value_and_error(self) -> (Self::Value, Option<Self::Error>) {
        match self {
            Ok(v) => (v, None),
            Err((v, error)) => (v, Some(error)),
        }
    }
}

pub trait ResultUnwrapValueExt {
    type Value;
    fn unwrap_value(self) -> Self::Value;
}

impl<T> ResultUnwrapValueExt for Result<T, T> {
    type Value = T;

    #[inline]
    fn unwrap_value(self) -> T {
        match self {
            Ok(v) => v,
            Err(v) => v,
        }
    }
}
