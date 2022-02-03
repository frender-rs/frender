use std::{cell::RefCell, rc::Rc};

pub fn use_ref_cell<T: 'static>(initial_value: T) -> Rc<RefCell<T>> {
    use_ref_cell_with(|| initial_value)
}

pub fn use_ref_cell_with<T: 'static, F: FnOnce() -> T>(get_initial_value: F) -> Rc<RefCell<T>> {
    super::use_memo_no_dep::<RefCell<T>, _, _>(|| RefCell::new(get_initial_value()))
}
