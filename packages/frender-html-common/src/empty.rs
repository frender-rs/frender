use frender_common::Empty;

impl crate::MaybeStringValue for Empty {
    type StringValue = async_str_iter::never::Never;

    fn maybe_string_value(Self: Self) -> Option<Self::StringValue> {
        None
    }
}

impl crate::IntoOneStringOrEmpty for Empty {
    type OneStringOrEmpty = async_str_iter::empty::Empty;

    fn into_one_string_or_empty(Self: Self) -> Self::OneStringOrEmpty {
        async_str_iter::empty::Empty
    }
}
