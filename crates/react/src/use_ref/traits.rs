use std::rc::Rc;

pub trait ReadRef<T> {
    fn current(&self) -> T;
}

pub trait WriteRef<T> {
    fn set_current(&self, v: T);
}

pub trait MutableRef<T>: WriteRef<T> + ReadRef<T> {}

impl<T, S: WriteRef<T> + ReadRef<T>> MutableRef<T> for S {}

impl<T, F: ?Sized + Fn(T)> WriteRef<T> for F {
    fn set_current(&self, v: T) {
        self(v)
    }
}

impl<T, F: ?Sized + Fn(T)> WriteRef<T> for Rc<F> {
    fn set_current(&self, v: T) {
        self(v)
    }
}
