use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use convert_js::{FromJs, ToJs};
use wasm_bindgen::JsCast;

pub fn use_ref_cell_value<T: 'static>(initial_value: T) -> Rc<RefCell<T>> {
    use_ref_cell(|| initial_value)
}

pub fn use_ref_cell<T: 'static, F: FnOnce() -> T>(get_initial_value: F) -> Rc<RefCell<T>> {
    super::use_memo_no_dep::<RefCell<T>, _, _>(|| RefCell::new(get_initial_value()))
}
