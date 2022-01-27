use std::rc::Rc;

use convert_js::WrapJsCast;
use wasm_bindgen::JsCast;

pub trait SimpleTake<T> {
    fn simple_take(self) -> T;
}

impl<T> SimpleTake<T> for T {
    #[inline]
    fn simple_take(self) -> T {
        self
    }
}

impl<T> SimpleTake<Rc<T>> for T {
    #[inline]
    fn simple_take(self) -> Rc<T> {
        Rc::new(self)
    }
}

impl<T: JsCast> SimpleTake<WrapJsCast<T>> for T {
    #[inline]
    fn simple_take(self) -> WrapJsCast<T> {
        WrapJsCast(self)
    }
}

impl<T: JsCast> SimpleTake<T> for WrapJsCast<T> {
    #[inline]
    fn simple_take(self) -> T {
        self.0
    }
}
