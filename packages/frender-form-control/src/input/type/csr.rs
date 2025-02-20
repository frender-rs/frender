use frender_common::reactive_value::non_reactive::CachedNonReactiveValue;

use super::InputType;

pub trait CsrInputType: InputType<InputTypeStr: CachedNonReactiveValue<str>> {}

impl<T: ?Sized> CsrInputType for T where T: InputType<InputTypeStr: CachedNonReactiveValue<str>> {}
