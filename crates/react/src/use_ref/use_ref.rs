use std::{cell::RefCell, rc::Rc};

use crate::{IntoRc, ReadRef, SimpleInto, TakeRc, WriteRef};

pub fn use_ref<T: 'static + ?Sized, D: IntoRc<T>>(initial_value: D) -> MutableRefRc<T> {
    let rc = crate::use_ref_cell(initial_value.into_rc());
    MutableRefRc(rc)
}

pub fn use_ref_with<T: 'static + ?Sized, F: TakeRc<T>>(get_initial_value: F) -> MutableRefRc<T> {
    let rc = crate::use_ref_cell_with(|| get_initial_value.take_rc());
    MutableRefRc(rc)
}

pub struct MutableRefRc<T: ?Sized>(Rc<RefCell<Rc<T>>>);

impl<T: ?Sized> ReadRef<Rc<T>> for MutableRefRc<T> {
    fn current(&self) -> Rc<T> {
        let a = self.0.borrow();
        Rc::clone(&a)
    }
}

impl<T: ?Sized, S: SimpleInto<Rc<T>>> WriteRef<S> for MutableRefRc<T> {
    fn set_current(&self, v: S) {
        let v: Rc<T> = v.simple_into();
        let mut a = self.0.borrow_mut();
        *a = v
    }
}

impl<T> crate::SafeIntoJsRuntime for MutableRefRc<T>
where
    dyn Fn(T): wasm_bindgen::closure::WasmClosure,
    T: 'static,
{
    fn safe_into_js_runtime(self) -> crate::PassedToJsRuntime {
        crate::WrapFn::new(move |v| self.set_current(v)).safe_into_js_runtime()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::super::WriteRef;
    use super::MutableRefRc;

    #[test]
    fn auto_impl_write_ref() {
        let _func_rc: fn(&MutableRefRc<i32>, v: Rc<i32>) =
            <MutableRefRc<i32> as WriteRef<Rc<i32>>>::set_current;
        let _func: fn(&MutableRefRc<i32>, v: i32) =
            <MutableRefRc<i32> as WriteRef<i32>>::set_current;
    }
}
