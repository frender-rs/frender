use frender_common::strings::SsrStr;

use super::InputType;

pub trait SsrInputType: InputType<InputTypeStr: SsrStr> {}

impl<T: ?Sized> SsrInputType for T where T: InputType<InputTypeStr: SsrStr> {}
