use std::{cell::RefCell, rc::Rc};

use crate::{IntoRc, IntoRefValue, ReadRef, WriteRef};

pub fn use_ref<T: 'static + ?Sized>(initial_value: Rc<T>) -> MutableRefRc<T> {
    let rc = crate::use_ref_cell(initial_value);
    MutableRefRc(rc)
}

pub fn use_ref_with<T: 'static + ?Sized, F: FnOnce() -> Rc<T>>(
    get_initial_value: F,
) -> MutableRefRc<T> {
    let ref_cell = super::use_ref_cell_with::<Rc<T>, _>(move || get_initial_value().into_rc());
    MutableRefRc(ref_cell)
}

/// `value` will be the initial value.
/// And in each render, if `value` changes,
/// the [`MutableRefRc`] will be set as it.
///
/// ```no_run
/// # fn get_value() { Rc::new(0) }
/// let value: Rc<i32> = get_value();
/// react::use_ref_set_as(value);
/// ```
///
/// The above rust code is equivalent to js code:
///
/// ```js
/// const value = getValue();
/// const refValue = React.useRef(value);
/// if (refValue.current != value) {
///     refValue.current = value;
/// }
/// ```
pub fn use_ref_set_as<T: 'static + ?Sized>(value: Rc<T>) -> MutableRefRc<T> {
    let ref_value = use_ref(Rc::clone(&value));
    if !Rc::ptr_eq(&*ref_value.0 .0.borrow(), &value) {
        ref_value.set_current(value)
    }
    ref_value
}

pub struct MutableRefRc<T: ?Sized>(super::ReadRefRc<RefCell<Rc<T>>>);

impl<T: ?Sized + std::fmt::Debug> std::fmt::Debug for MutableRefRc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0 .0.borrow();
        let v = &**v;
        f.debug_tuple("MutableRefRc").field(&v).finish()
    }
}

impl<T: ?Sized> PartialEq for MutableRefRc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Eq for MutableRefRc<T> {}

impl<T: ?Sized> Clone for MutableRefRc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> ReadRef<Rc<T>> for MutableRefRc<T> {
    #[inline]
    fn current(&self) -> Rc<T> {
        let a = self.0 .0.borrow();
        Rc::clone(&*a)
    }
}

impl<T: ?Sized, S: IntoRefValue<Rc<T>>> WriteRef<S> for MutableRefRc<T> {
    #[inline]
    fn set_current(&self, v: S) {
        let v: Rc<T> = v.into_ref_value();
        let mut a = self.0 .0.borrow_mut();
        *a = v;
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
