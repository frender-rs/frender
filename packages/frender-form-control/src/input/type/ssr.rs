use frender_reactive_value::static_or_into_static_str::StaticOrIntoStaticStr;

use super::InputType;

pub trait SsrInputType: InputType<InputTypeStr: StaticOrIntoStaticStr> {}

impl<T: ?Sized> SsrInputType for T where T: InputType<InputTypeStr: StaticOrIntoStaticStr> {}
