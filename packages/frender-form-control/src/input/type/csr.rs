use frender_reactive_value::{non_reactive::CachedNonReactiveValue, value_kind::KindOfTempRef};

use super::InputType;

pub trait CsrInputType:
    InputType<InputTypeStr: CachedNonReactiveValue<KindOfTempRef<str>>>
{
}

impl<T: ?Sized> CsrInputType for T where
    T: InputType<InputTypeStr: CachedNonReactiveValue<KindOfTempRef<str>>>
{
}
