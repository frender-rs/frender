use std::{cell::RefCell, rc::Rc};

pub fn use_ref_cell<T: 'static>(initial_value: T) -> super::ReadRefRc<RefCell<T>> {
    super::use_ref_cell_with(move || initial_value)
}

pub fn use_ref_cell_with<T: 'static, F: FnOnce() -> T>(
    get_initial_value: F,
) -> super::ReadRefRc<RefCell<T>> {
    super::use_ref_readonly_with(move || Rc::new(RefCell::new(get_initial_value())))
}
