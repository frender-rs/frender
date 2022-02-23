use std::rc::Rc;

use convert_js::WrapJsCast;
use wasm_bindgen::JsCast;

pub trait IntoRefValue<T> {
    fn into_ref_value(self) -> T;
}

impl<T> IntoRefValue<T> for T {
    #[inline]
    fn into_ref_value(self) -> T {
        self
    }
}

impl<T> IntoRefValue<Rc<T>> for T {
    #[inline]
    fn into_ref_value(self) -> Rc<T> {
        Rc::new(self)
    }
}

impl<T: JsCast> IntoRefValue<WrapJsCast<T>> for T {
    #[inline]
    fn into_ref_value(self) -> WrapJsCast<T> {
        WrapJsCast(self)
    }
}

impl<T: JsCast> IntoRefValue<T> for WrapJsCast<T> {
    #[inline]
    fn into_ref_value(self) -> T {
        self.0
    }
}

impl<T> IntoRefValue<Box<T>> for T {
    #[inline]
    fn into_ref_value(self) -> Box<T> {
        Box::new(self)
    }
}

impl<T> IntoRefValue<T> for Box<T> {
    #[inline]
    fn into_ref_value(self) -> T {
        *self
    }
}
