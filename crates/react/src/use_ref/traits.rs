use std::rc::Rc;
use wasm_bindgen::closure::Closure;

pub trait ReadRef<T> {
    fn current(&self) -> T;
}

pub trait WriteRef<T> {
    fn set_current(&self, v: T);
}

pub trait MutableRef<T>: WriteRef<T> + ReadRef<T> {}

impl<T, S: WriteRef<T> + ReadRef<T>> MutableRef<T> for S {}

impl<T: 'static> WriteRef<T> for Closure<dyn Fn(T)> {
    fn set_current(&self, _: T) {
        panic!(
            "closure as WriteRef should only be called in js runtime: {:?}",
            self
        );
    }
}

impl<T> WriteRef<T> for Box<dyn Fn(T)>
where
    Self: crate::TryBorrowToJsRuntime,
{
    fn set_current(&self, v: T) {
        self(v)
    }
}

impl<T> WriteRef<T> for Rc<dyn Fn(T)>
where
    Self: crate::TryBorrowToJsRuntime,
{
    fn set_current(&self, v: T) {
        self(v)
    }
}
