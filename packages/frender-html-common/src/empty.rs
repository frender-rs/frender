use frender_common::Empty;

impl crate::MaybeStringValue for Empty {
    type StringValue = async_str_iter::never::Never;

    fn maybe_string_value(Self: Self) -> Option<Self::StringValue> {
        None
    }
}
