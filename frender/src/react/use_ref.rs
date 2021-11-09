use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use react_sys::MutableRefObjectUsize;

pub struct MutableRefObject<T>(MutableRefObjectUsize, PhantomData<T>);

pub fn use_ref_value<T: 'static>(initial_value: T) -> Rc<RefCell<T>> {
    use_ref(|| initial_value)
}

pub fn use_ref<T: 'static, F: FnOnce() -> T>(get_initial_value: F) -> Rc<RefCell<T>> {
    super::use_memo_no_dep::<RefCell<T>, _, _>(|| RefCell::new(get_initial_value()))
}
