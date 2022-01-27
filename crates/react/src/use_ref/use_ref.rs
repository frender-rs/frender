use std::{cell::RefCell, rc::Rc};

use crate::{IntoRc, ReadRef, SimpleTake, TakeRc, WriteRef};

pub fn use_ref_value<T: 'static + ?Sized, D: IntoRc<T>>(initial_value: D) -> MutableRefRc<T> {
    let rc = crate::use_ref_cell_value(initial_value.into_rc());
    MutableRefRc(rc)
}

pub fn use_ref<T: 'static + ?Sized, F: TakeRc<T>>(get_initial_value: F) -> MutableRefRc<T> {
    let rc = crate::use_ref_cell(|| get_initial_value.take_rc());
    MutableRefRc(rc)
}

pub struct MutableRefRc<T: ?Sized>(Rc<RefCell<Rc<T>>>);

impl<T: ?Sized> ReadRef<Rc<T>> for MutableRefRc<T> {
    fn current(&self) -> Rc<T> {
        let a = self.0.borrow();
        Rc::clone(&a)
    }
}

impl<T: ?Sized, S: SimpleTake<Rc<T>>> WriteRef<S> for MutableRefRc<T> {
    fn set_current(&self, v: S) {
        let v: Rc<T> = v.simple_take();
        let mut a = self.0.borrow_mut();
        *a = v
    }
}

crate::impl_pass_to_js_runtime! {
    [
        try_borrow: None
        try_into: Err
    ]
    { T: ?Sized } MutableRefRc<T>
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::super::WriteRef;
    use super::MutableRefRc;

    #[test]
    fn auto_impl_write_ref() {
        let func_rc: fn(&MutableRefRc<i32>, v: Rc<i32>) =
            <MutableRefRc<i32> as WriteRef<Rc<i32>>>::set_current;
        let func: fn(&MutableRefRc<i32>, v: i32) =
            <MutableRefRc<i32> as WriteRef<i32>>::set_current;
    }
}
