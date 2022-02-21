use std::rc::Rc;

use convert_js::WrapJsCast;
use wasm_bindgen::JsCast;

pub trait SimpleInto<T> {
    fn simple_into(self) -> T;
}

impl<T> SimpleInto<T> for T {
    #[inline]
    fn simple_into(self) -> T {
        self
    }
}

impl<T> SimpleInto<Rc<T>> for T {
    #[inline]
    fn simple_into(self) -> Rc<T> {
        Rc::new(self)
    }
}

impl<T: JsCast> SimpleInto<WrapJsCast<T>> for T {
    #[inline]
    fn simple_into(self) -> WrapJsCast<T> {
        WrapJsCast(self)
    }
}

impl<T: JsCast> SimpleInto<T> for WrapJsCast<T> {
    #[inline]
    fn simple_into(self) -> T {
        self.0
    }
}

impl<T> SimpleInto<Box<T>> for T {
    #[inline]
    fn simple_into(self) -> Box<T> {
        Box::new(self)
    }
}

impl<T> SimpleInto<T> for Box<T> {
    #[inline]
    fn simple_into(self) -> T {
        *self
    }
}
